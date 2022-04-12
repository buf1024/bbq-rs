from typing import Optional, List, Tuple, Union


from qstrategy.base_strategy import BaseStrategy
from qstrategy.position import Position
from qstrategy.entrust import Entrust

class Broker(BaseStrategy):
    """
    券商类接口
    接收事件
    1. entrust 买/卖/撤销委托事件
    
    券商产生的事件:
    broker_event
    1. entrust 委托提交/委托(买,卖)成交/撤销事件
    2. fund_sync 资金同步事件 总资金，可用资金，持仓市值
    3. pos_sync 持仓同步事件
    """
    
    evt_entrust, evt_fund_sync, evt_pos_sync = 'entrust', 'fund_sync', 'pos_sync'

    def __init__(self):
        super().__init__()

        funcs = dict(
            on_entrust=self.on_entrust,
            on_poll=self.on_poll
        )

        self.funcs.update(funcs)
        
        self.broker_entrust = []
        self.broker_pos = []
        self.broker_fund = None
        
    def on_poll(self, _1, _2) -> Optional[List]:
        events = []
        for entrust in self.broker_entrust:
            events.append({'entrust': entrust})
        
        if len(self.broker_entrust) > 0:
            events.append({'position': self.broker_pos})
            
        if self.broker_fund is not None:
            events.append({'fund_sync': self.broker_fund})
        
        self.log.info('broker on_poll, events = {}'.format(len(events)))

        if len(events) > 0:
            self.broker_entrust = []
            self.broker_pos = []
            self.broker_fund = None
            
            return events
        
    def emit(self, typ: str, event: Union[List[Position], Entrust, Tuple]):
        """
        @param typ: entrust/fund_sync/pos_sync
        """
        if isinstance(typ, str) and len(typ) > 0:
            dog = typ[0].lower()
            if dog == 'e':
                self.broker_entrust.append({'entrust': event})
            elif dog == 'p':
                self.broker_pos = event
            elif dog == 'f':
                self.broker_fund = event

    def on_entrust(self, evt, payload) -> bool:
        """
        券商委托调用
        
        :return:
        """
        return None

