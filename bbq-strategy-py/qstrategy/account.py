

class Account:
    def __init__(self):
        self.cash_init = 0.0
        self.cash_available = 0.0
        self.cash_frozen = 0.0
        self.total_net_value = 0.0

        self.total_hold_value = 0.0
        self.cost = 0    # 持仓陈本
        self.profit = 0  # 持仓盈亏
        self.profit_rate = 0  # 持仓盈比例

        self.close_profit = 0  # 平仓盈亏

        self.total_profit = 0  # 总盈亏
        self.total_profit_rate = 0  # 总盈亏比例

        self.cost = 0    # 持仓陈本
        self.profit = 0  # 持仓盈亏
        self.profit_rate = 0  # 持仓盈比例

        self.broker_fee = 0.00025
        self.transfer_fee = 0.00002
        self.tax_fee = 0.001

        self.start_time = None
        self.end_time = None

        self.position = {}
        self.entrust = {}

        # 成交 backtest
        self.deal = []
        self.signal = []

        self.acct_his = []
