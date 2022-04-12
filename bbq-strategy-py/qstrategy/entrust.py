from datetime import datetime


class Entrust:
    st_init, st_commit, st_deal, st_part_deal, st_cancel = 'init', 'commit', 'deal', 'part_deal', 'cancel'
    typ_buy, typ_sell, typ_cancel = 'buy', 'sell', 'cancel'
    def __init__(self, js):
        self.entrust_id = js['entrust_id']
        self.name = js['name']
        self.code = js['code']
        self.volume_deal = js['volume_deal']
        self.volume_cancel = js['volume_cancel']
        self.volume = js['volume']
        self.price = js['price']
        self.status = js['status']
        self.entrust_type = js['entrust_type']
        self.desc = js['desc']
        self.broker_entrust_id = js['broker_entrust_id'] if 'broker_entrust_id' in js else None

        self.time = None
        if js['time'] is not None and isinstance(js['time'], str):
            s = js['time'][:-7]
            self.time = js['time']
            self.time = datetime.strptime(s, '%Y-%m-%dT%H:%M:%S')

    def to_dict(self):
        d = {}
        d['entrust_id'] = self.entrust_id
        d['name'] = self.name
        d['code'] = self.code

        d['volume_deal'] = self.volume_deal
        d['volume_cancel'] = self.volume_cancel
        d['volume'] = self.volume
        d['price'] = self.price
        d['status'] = self.status
        d['entrust_type'] = self.entrust_type
        d['desc'] = self.desc
        d['broker_entrust_id'] = self.broker_entrust_id

        d['time'] = None if self.time is None else self.time.strftime(
            "%Y-%m-%dT%H:%M:%S.%f")

        return d
