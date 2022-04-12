from datetime import datetime

class Signal:
    sig_sell, sig_buy, sig_cancel = 'sell', 'buy', 'cancel'

    def __init__(self, *, signal: str = '',
                 code: str = '', name: str = '',
                 price: float = 0.0, volume: int = 0,
                 desc: str = '', entrust_id: str = None,
                 time: datetime = None):
        self.signal = signal  # sell, buy, cancel

        self.name = name  # 股票名称
        self.code = code  # 股票代码
        self.time = time

        self.price = price
        self.volume = volume
        self.desc = desc

        self.entrust_id = None  # sell / cancel 有效

    def to_dict(self):
        d = {}
        d['signal'] = self.signal
        d['name'] = self.name
        d['code'] = self.code
        d['time'] = None if self.time is None else self.time.strftime(
            "%Y-%m-%dT%H:%M:%S.%f")
        d['price'] = self.price
        d['volume'] = self.volume
        d['desc'] = self.desc
        d['entrust_id'] = self.entrust_id
        return d
