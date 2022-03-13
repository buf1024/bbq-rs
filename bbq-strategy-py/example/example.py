from qstrategy.strategy import *


class ExampleStrategy(Strategy):
    def on_init(self, opt: Optional[Dict]) -> bool:
        pass

    def on_destroy(self) -> bool:
        pass

    def on_open(self, evt, payload):
        pass

    def on_close(self, evt, payload) -> Signal:
        pass

    def on_quot(self, evt, payload) -> Optional[List[Signal]]:
        self.log.info('strategy on_quot: evt={}, payload={}'.format(evt, payload))
        return None
