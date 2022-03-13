import time
from abc import ABC
from collections import namedtuple
from functools import wraps

import pandas as pd
import pymongo
from pymongo.errors import ServerSelectionTimeoutError, AutoReconnect

import qstrategy.log as log


class MongoDB(ABC):
    def __init__(self, uri='mongodb://localhost:27017/'):
        self.log = log.get_logger(self.__class__.__name__)
        self.client = None

        self.uri = uri

    def _retry(func):
        @wraps(func)
        def wrapper(self, *args, **kwargs):
            attempts, sleep = 5, 1
            for i in range(attempts):
                try:
                    return func(self, *args, **kwargs)
                except (ServerSelectionTimeoutError, AutoReconnect) as e:
                    backoff = sleep ** (i + 1)
                    self.log.error('loss db connection, reconnect {}s later'.format(backoff))
                    if i + 1 == attempts:
                        raise e
                    time.sleep(backoff)
                    self.init()
                except Exception as e:
                    raise e

        return wrapper

    def test_coll(self):
        return None

    def init(self) -> bool:
        try:
            self.client = pymongo.MongoClient(self.uri)
            test_coll = self.test_coll()
            if test_coll is not None:
                test_coll.count_documents({})
        except Exception as e:
            self.log.error(type(e))
            return False

        return True

    def get_coll(self, db: str, col: str):
        if self.client is None:
            return None
        return self.client[db][col]

    @_retry
    def do_load(self, coll, filter=None, projection=None, skip=0, limit=0, sort=None, to_frame=True):
        cursor = coll.find(filter=filter, projection=projection, skip=skip, limit=limit, sort=sort)
        if cursor is not None:
            data = [item for item in cursor]
            # data = cursor.to_list(None)
            cursor.close()
            if to_frame:
                df = pd.DataFrame(data=data, columns=projection)
                if not df.empty:
                    if '_id' in df.columns:
                        df.drop(columns=['_id'], inplace=True)
                    return df
            else:
                if len(data) > 0:
                    for item in data:
                        del item['_id']
                return data
        return None

    @_retry
    def do_update(self, coll, filter=None, update=None, upsert=True):
        if update is None:
            return None
        res = coll.update_one(filter, {'$set': update}, upsert=upsert)
        return res.matched_count if res.matched_count > 0 else (
            res.upserted_id if res.upserted_id is not None else 0)

    @_retry
    def do_update_many(self, coll, filter=None, update=None, upsert=True):
        if update is None:
            return None
        res = coll.update_many(filter, {'$set': update}, upsert=upsert)
        return res.matched_count if res.matched_count > 0 else (
            res.upserted_id if res.upserted_id is not None else 0)

    def do_batch_update(self, data, func):
        upsert_list = []
        for item in data.to_dict('records'):
            coll, filter, update = func(item)
            upsert = self.do_update(coll, filter=filter, update=update)
            if upsert is None:
                continue
            if isinstance(upsert, list):
                upsert_list = upsert_list + upsert
            else:
                upsert_list.append(upsert)
        return upsert_list if len(upsert_list) > 0 else None

    @_retry
    def do_delete(self, coll, filter=None, just_one=True):
        res = None
        if just_one:
            res = coll.delete_one(filter)
        else:
            if filter is not None:
                res = coll.delete_many(filter)
            else:
                res = coll.drop()
        return 0 if res is None else res.deleted_count

    @_retry
    def do_insert(self, coll, data):
        inserted_ids = []
        if data is not None and not data.empty:
            docs = data.to_dict('records')
            result = coll.insert_many(docs)
            inserted_ids = result.inserted_ids
        return inserted_ids
