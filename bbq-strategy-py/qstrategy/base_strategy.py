import json
from tkinter import N
from typing import Dict, Optional
from datetime import datetime
import uuid
from qstrategy.account import Account
import qstrategy.log as log
from qstrategy.trade_signal import Signal
from qstrategy.js_encoder import MyJsonEncoder
import os
import traceback


class BaseStrategy:
    def __init__(self):
        self.log = None
        self.account = None

        self.funcs = dict(
            init=self.init,
            destroy=self.destroy,
        )

    def __call__(self, account, func, evt, payload):
        if func in self.funcs:
            try:
                if account is not None and len(account) > 0:
                    self.account = Account(json.loads(account))
                if func in self.funcs:
                    fn = self.funcs[func]
                    payload = json.loads(
                        payload) if payload is not None and len(payload) > 0 else None
                    rst = fn(evt, payload)
                    if func not in ['init', 'destroy'] and rst is not None:
                        if isinstance(rst, str) or isinstance(rst, Signal):
                            rst = [rst]

                        if isinstance(rst, list):
                            rst = json.dumps(rst, cls=MyJsonEncoder)
                        else:
                            rst = None
                    return rst
            except Exception as e:
                callstack = traceback.format_exc()

                print("exception: {}, callstack: {}".format(e, callstack))
                self.log.error("exception: {}, callstack: {}".format(e, callstack))
                if func in ['init', 'destroy']:
                    return False

    @staticmethod
    def get_uuid():
        return str(uuid.uuid4()).replace('-', '')
    
    def buy_signal(self, *,
                   code: str = '', name: str = '',
                   price: float = 0.0, volume: int = 0,
                   desc: str = '', entrust_id: str = None,
                   time: datetime = None):
        if time is None:
            # todo
            time = datetime.now()

        return Signal(signal=Signal.sig_buy,
                      code=code, name=name,
                      price=price, volume=volume,
                      desc=desc, entrust_id=entrust_id, time=time)

    def sell_signal(self, *,
                    code: str = '', name: str = '',
                    price: float = 0.0, volume: int = 0,
                    desc: str = '', entrust_id: str = None,
                    time: datetime = None):
        if time is None:
            # todo
            time = datetime.now()
        return Signal(signal=Signal.sig_sell,
                      code=code, name=name,
                      price=price, volume=volume,
                      desc=desc, entrust_id=entrust_id, time=time)

    def cancel_signal(self, *,
                      code: str = '', name: str = '',
                      price: float = 0.0, volume: int = 0,
                      desc: str = '', entrust_id: str = None,
                      time: datetime = None):
        if time is None:
            # todo
            time = datetime.now()

        return Signal(signal=Signal.sig_cancel,
                      code=code, name=name,
                      price=price, volume=volume,
                      desc=desc, entrust_id=entrust_id, time=time)

    def init(self, _, opt: Optional[Dict]) -> bool:
        log_path, log_level = None, 'info'
        if 'log_path' in opt:
            log_path = opt['log_path']
            
            try:
                getattr(self, 'on_quot')
                log_path = '{}/{}'.format(log_path, 'strategy_py')
            except:
                pass
            
            try:
                getattr(self, 'on_risk')
                log_path = '{}/{}'.format(log_path, 'risk_py')
            except:
                pass
            
            try:
                getattr(self, 'on_entrust')
                log_path = '{}/{}'.format(log_path, 'broker_py')
            except:
                pass
            
            os.makedirs(log_path, exist_ok=True)
            log_path = '{}/{}.log'.format(log_path, self.name())
        if 'log_level' in opt:
            log_level = opt['log_level']
        print('log_path={}'.format(log_path))
        log.setup_logger(file=log_path, level=log_level)
        self.log = log.get_logger(self.__class__.__name__)

        return self.on_init(opt)

    def destroy(self, _1, _2) -> bool:
        return self.on_destroy()

    def on_init(self, opt: Optional[Dict]) -> bool:
        return True

    def on_destroy(self) -> bool:
        return True

    def name(self) -> str:
        return self.__class__.__name__
