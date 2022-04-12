from typing import Dict, List, Optional
from qstrategy.risk import Risk
from qstrategy.trade_signal import Signal


class ExampleRisk(Risk):
    def on_init(self, opt: Optional[Dict]) -> bool:
        self.log.info('risk account: {}, on_init'.format(
            self.account.account_id))
        return True

    def on_destroy(self) -> bool:
        self.log.info('risk on_destroy')
        return True

    def on_risk(self, evt, payload) -> Optional[List[Signal]]:
        self.log.info('risk on_risk')
        # pass
