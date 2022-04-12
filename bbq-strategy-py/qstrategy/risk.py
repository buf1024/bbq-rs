from typing import Optional, List, Union


from qstrategy.base_strategy import BaseStrategy
from qstrategy.trade_signal import Signal

class Risk(BaseStrategy):
    """
    风控类接口，可产生信号:
    1. signal/[signal, ...] 取消委托/委托买/委托卖
    """
    

    def __init__(self):
        super().__init__()

        funcs = dict(
            on_risk=self.on_risk
        )

        self.funcs.update(funcs)

    def on_risk(self, evt, payload) -> Optional[Union[Signal, List[Signal]]]:
        """
        风控回调
        
        :return:
        """
        return None
