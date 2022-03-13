from typing import Dict, Optional, List
import qstrategy.log as log


class Context:
    pass


class Events:
    pass


class Signal:
    pass


class Strategy:
    """
    策略类接口，可产生信号:
    1. evt_sig_cancel 取消委托
    2. evt_sig_buy 委托买
    3. evt_sig_sell 委托卖
    4. evt_quot_codes 订阅行情
    """

    def __init__(self):
        self.log = None
        self.ctx = None

        self.funcs = dict(
            init=self.init,
            destroy=self.destroy,
            on_open=self.on_open,
            on_close=self.on_close,
            on_quot=self.on_quot
        )

    def __call__(self, data, func, evt, payload):
        if func in self.funcs:
            self.update_context(data)
            func = self.funcs[func]
            rst = func(evt, payload)
            if func not in ['init', 'destroy'] and rst is not None:
                rst = self.encode_json(rst)
            return rst

    def update_context(self, data):
        pass

    def decode_json(self, data):
        pass

    def encode_json(self, data):
        pass

    def init(self, opt: Optional[Dict]) -> bool:
        self.log = log.get_logger(self.__class__.__name__)
        return self.on_init(opt)

    def destroy(self) -> bool:
        return self.on_destroy()

    # 以下函数重写
    def name(self) -> str:
        return self.__class__.__name__

    def on_init(self, opt: Optional[Dict]) -> bool:
        self.log = log.get_logger(self.__class__.__name__)

        return True

    def on_destroy(self) -> bool:
        return True

    def on_open(self, evt, payload):
        """
        开始事件回调
        :param evt: evt_start/evt_morning_start/evt_noon_start
                    程序开始/交易日早市开始/交易日午市开始
        :param payload:
        :return:
        """
        pass

    def on_close(self, evt, payload) -> Signal:
        """
        结束事件回调
        :param evt: evt_end/evt_morning_end/evt_noon_end
                    程序结束/交易日早市结束/交易日午市结束
        :param payload:
        :return:
        """
        pass

    def on_quot(self, evt, payload) -> Optional[List[Signal]]:
        self.log.info('strategy on_quot: evt={}, payload={}'.format(evt, payload))
        return None
