from qstrategy.mongodb import MongoDB
import pandas as pd


class FundDB(MongoDB):
    _meta = {
        # 基金信息
        'fund_info': {
            'code': '基金代码', 'name': '基金简称', 'type': '基金类型'
        },
        # 基金净值信息
        'fund_net': {
            'code': '基金代码', 'trade_date': '交易日', 'net': '净值', 'net_acc': '累计净值',
            'rise': '日增长率', 'apply_status': '申购状态', 'redeem_status': '赎回状态'
        },
        # 场内基金日线数据
        'fund_daily': {'code': '代码', 'trade_date': '交易日', 'close': '收盘价', 'open': '开盘价', 'high': '最高价', 'low': '最低价',
                       'volume': '成交量(股)', 'turnover': '换手率'},
    }
    _db = 'bbq_fund_db'  # 基金数据库

    def __init__(self, uri='mongodb://localhost:27017/'):
        super().__init__(uri)

    @property
    def fund_info(self):
        return self.get_coll(self._db, 'fund_info')

    @property
    def fund_net(self):
        return self.get_coll(self._db, 'fund_net')

    @property
    def fund_daily(self):
        return self.get_coll(self._db, 'fund_daily')

    def test_coll(self):
        return self.fund_info

    def load_fund_info(self, **kwargs):
        self.log.debug('加载基金列表, kwargs={}'.format(kwargs))
        df = self.do_load(self.fund_info, **kwargs)
        self.log.debug('加载基金列表成功, size={}'.format(df.shape[0] if df is not None else 0))
        return df

    def save_fund_info(self, data):
        self.log.debug('保存基金列表, count = {}'.format(data.shape[0]))
        inserted_ids = self.do_batch_update(data=data,
                                            func=lambda x: (self.fund_info, {'code': x['code']}, x))
        self.log.debug('保存基金列表成功, size = {}'.format(len(inserted_ids) if inserted_ids is not None else 0))
        return inserted_ids

    def load_fund_net(self, **kwargs):
        self.log.debug('加载基金净值数据, kwargs={}'.format(kwargs))
        df = self.do_load(self.fund_net, **kwargs)
        self.log.debug('加载基金净值数据,成功, size={}'.format(df.shape[0] if df is not None else 0))
        return df

    def save_fund_net(self, data: pd.DataFrame):
        count = data.shape[0] if data is not None else 0
        inserted_ids = []
        self.log.debug('保存基金净值数据, count = {} ...'.format(data.shape[0]))
        if count > 0:
            inserted_ids = self.do_insert(coll=self.fund_net, data=data)
        self.log.debug('保存基金净值成功, size = {}'.format(len(inserted_ids) if inserted_ids is not None else 0))
        return inserted_ids

    def load_fund_daily(self, **kwargs):
        self.log.debug('加载场内基金日线数据, kwargs={}'.format(kwargs))
        df = self.do_load(self.fund_daily, **kwargs)
        self.log.debug('加载场内基金日线数据,成功, size={}'.format(df.shape[0] if df is not None else 0))
        return df

    def save_fund_daily(self, data: pd.DataFrame):
        count = data.shape[0] if data is not None else 0
        inserted_ids = []
        self.log.debug('保存场内基金日线数据, count = {} ...'.format(data.shape[0]))
        if count > 0:
            inserted_ids = self.do_insert(coll=self.fund_daily, data=data)
        self.log.debug('保存场内基金日线数据成功, size = {}'.format(len(inserted_ids) if inserted_ids is not None else 0))
        return inserted_ids


# if __name__ == '__main__':
#
#     def test_fund_net(db):
#         df = db.load_fund_net(filter={'code': '159702'})
#         print('test_fund_net db:\n')
#         print(df.head())
#     db = FundDB()
#     db.init()
#     test_fund_net(db)
