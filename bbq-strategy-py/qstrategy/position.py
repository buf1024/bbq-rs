from datetime import datetime
import sys


class Position:
    def __init__(self, js):
        self.position_id = js['position_id']
        self.name = js['name']
        self.code = js['code']
        self.volume = js['volume']
        self.volume_available = js['volume_available']
        self.fee = js['fee']
        self.price = js['price']
        self.profit_rate = js['profit_rate']
        self.max_profit_rate = js['max_profit_rate']
        self.min_profit_rate = js['min_profit_rate']
        self.profit = js['profit']
        self.max_profit = js['max_profit']
        self.min_profit = js['min_profit']
        self.now_price = js['now_price']
        self.max_price = js['max_price']
        self.min_price = js['min_price']
        self.max_profit_time = None
        self.min_profit_time = None
        if js['max_profit_time'] is not None and isinstance(js['max_profit_time'], str):
            s = js['max_profit_time'][:-7]
            self.max_profit_time = js['max_profit_time']
            self.max_profit_time = datetime.strptime(s, '%Y-%m-%dT%H:%M:%S')

        if js['min_profit_time'] is not None and isinstance(js['min_profit_time'], str):
            s = js['min_profit_time'][:-7]
            self.min_profit_time = js['min_profit_time']
            self.min_profit_time = datetime.strptime(s, '%Y-%m-%dT%H:%M:%S')

    def to_dict(self):
        d = {}
        d['position_id'] = self.position_id
        d['name'] = self.name
        d['code'] = self.code

        d['volume'] = self.volume
        d['volume_available'] = self.volume_available
        d['fee'] = self.fee
        d['price'] = self.price
        d['profit_rate'] = self.profit_rate
        d['max_profit_rate'] = self.max_profit_rate

        d['min_profit_rate'] = self.min_profit_rate
        d['profit'] = self.profit
        d['max_profit'] = self.max_profit
        d['min_profit'] = self.min_profit
        d['now_price'] = self.now_price
        d['max_price'] = self.max_price
        d['min_price'] = self.min_price

        d['max_profit_time'] = None if self.max_profit_time is None else self.max_profit_time.strftime(
            "%Y-%m-%dT%H:%M:%S.%f")
        d['min_profit_time'] = None if self.min_profit_time is None else self.min_profit_time.strftime(
            "%Y-%m-%dT%H:%M:%S.%f")

        return d
