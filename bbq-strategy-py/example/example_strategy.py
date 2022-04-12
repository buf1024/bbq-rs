from typing import Dict, List
from qstrategy.strategy import *


class ExampleStrategy(Strategy):
    def on_init(self, opt: Optional[Dict]) -> bool:
        self.log.info('account: {}, on_init'.format(self.account.account_id))
        return True

    def on_destroy(self) -> bool:
        self.log.info('strategy on_destroy')
        return True

    def on_open(self, evt, payload) -> Optional[Union[Signal, str, List[Signal], List[str]]]:
        self.log.info(
            'strategy on_open: evt={}, payload={}'.format(evt, payload))
        return 'sh600063'
        # pass

    def on_close(self, evt, payload) -> Optional[Union[Signal, str, List[Signal], List[str]]]:
        self.log.info(
            'strategy on_close: evt={}, payload={}'.format(evt, payload))
        # pass

    def on_quot(self, evt, payload) -> Optional[Union[Signal, str, List[Signal], List[str]]]:
        self.log.info(
            'strategy on_quot: evt={}, payload={}'.format(evt, payload))
        return self.buy_signal(code='sh600063', name='皖维高新')
        # pass
