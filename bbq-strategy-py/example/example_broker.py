from typing import Dict, Optional
from qstrategy.broker import Broker
from qstrategy.entrust import Entrust


class ExampleBroker(Broker):
    def on_init(self, opt: Optional[Dict]) -> bool:
        self.log.info('broker on_init')
        return True

    def on_destroy(self) -> bool:
        self.log.info('broker on_destroy')
        return True

    def on_entrust(self, evt, payload) -> bool:
        self.log.info('broker on_entrust: {}, {}'.format(evt, payload))
        entrust = Entrust(payload)
        if entrust.entrust_type == Entrust.typ_buy:
            entrust.broker_entrust_id = self.get_uuid()
            entrust.status = entrust.st_deal
            entrust.volume_deal = entrust.volume
            self.emit(self.evt_entrust, entrust)

        if entrust.entrust_type == Entrust.typ_sell:
            entrust.broker_entrust_id = self.get_uuid()
            entrust.status = entrust.st_deal
            entrust.volume_deal = entrust.volume
            self.emit(self.evt_entrust, entrust)

        if entrust.entrust_type == Entrust.typ_cancel:
            entrust.broker_entrust_id = self.get_uuid()
            entrust.status = entrust.st_cancel
            entrust.volume_cancel = entrust.volume_cancel
            self.emit(self.evt_entrust, entrust)
            
        return True
