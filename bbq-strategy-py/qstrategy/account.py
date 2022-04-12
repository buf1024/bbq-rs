from qstrategy import events
from qstrategy.entrust import Entrust
from qstrategy.position import Position

class Account:
    def __init__(self, js):
        self.account_id = js['account_id']
        self.cash_init = js['cash_init']
        self.cash_available = js['cash_available']
        self.cash_frozen = js['cash_frozen']
        self.total_net_value = js['total_net_value']

        self.total_hold_value = js['total_hold_value']
        self.cost = js['cost']    # 持仓陈本
        self.profit = js['profit']  # 持仓盈亏
        self.profit_rate = js['profit_rate']  # 持仓盈比例

        self.close_profit = js['close_profit']  # 平仓盈亏

        self.total_profit = js['total_profit']  # 总盈亏
        self.total_profit_rate = js['total_profit_rate']  # 总盈亏比例

        self.cost = js['cost']    # 持仓陈本
        self.profit = js['profit']  # 持仓盈亏
        self.profit_rate = js['profit_rate']  # 持仓盈比例

        self.broker_fee = js['broker_fee']
        self.transfer_fee = js['transfer_fee']
        self.tax_fee = js['tax_fee']
        
        self.position = {}
        if 'position' in js:
            for c, d_js in js['position'].items():
                self.position[c] = Position(d_js)
        
        self.entrust = []
        if 'entrust' in js:
            for d_js in js['entrust']:
                self.entrust.append(Entrust(d_js))

    def get_fee(self, typ, code, price, volume) -> float:
        total = price * volume
        broker_fee = total * self.broker_fee
        if broker_fee < 5:
            broker_fee = 5
        tax_fee = 0
        if typ == events.act_buy:
            if code.startswith('sh6'):
                tax_fee = total * self.transfer_fee
        elif typ == events.act_sell:
            if code.startswith('sz') or code.startswith('sh'):
                tax_fee = total * self.tax_fee
        return round(broker_fee + tax_fee, 4)

    def get_cost(self, typ, code, price, volume) -> float:
        fee = self.get_fee(typ=typ, code=code, price=price, volume=volume)
        return round(fee + price * volume, 4)

    