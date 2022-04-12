import json
from qstrategy.trade_signal import Signal
from qstrategy.position import Position
from qstrategy.entrust import Entrust


class MyJsonEncoder(json.JSONEncoder):
    def default(self, o):
        if isinstance(o, Signal) or isinstance(o, Position) or isinstance(o, Entrust):
            return o.to_dict()
        else:
            return json.JSONEncoder.default(self, o)
