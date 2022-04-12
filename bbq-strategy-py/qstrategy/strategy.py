from typing import Optional, List, Union


from qstrategy.base_strategy import BaseStrategy
from qstrategy.trade_signal import Signal

# quotation
#
# {"morning_start":
#     {"opts":{"kind":"stock","frequency":86400,"codes":["sh600063","sh601456","sh000001"],"start_date":"2021-03-01","end_date":"2022-03-01"},"time":"2022-03-01T09:30:00"}}
# {"quot":
#     {"sh600063":{"frequency":86400,"open":5.93,"high":5.96,"low":5.9,"close":5.96,"start":"2022-02-28 00:00:00","end":"2022-03-01 00:00:00",
#                  "quot":{"code":"sh600063","name":"","open":5.93,"pre_close":0.0,"now":5.9,"high":5.96,"low":5.85,"buy":5.9,"sell":5.9,"vol":15094500,"amount":0.0,"bid":[[0,0.0],[0,0.0],[0,0.0],[0,0.0],[0,0.0]],"ask":[[0,0.0],[0,0.0],[0,0.0],[0,0.0],[0,0.0]],"date":"2022-03-01","time":"2022-03-01 00:00:00"}},
#      "sh601456":{"frequency":86400,"open":14.71,"high":15.0,"low":14.91,"close":15.0,"start":"2022-02-28 00:00:00","end":"2022-03-01 00:00:00",
#                  "quot":{"code":"sh601456","name":"","open":14.71,"pre_close":0.0,"now":14.91,"high":15.0,"low":14.58,"buy":14.91,"sell":14.91,"vol":30642376,"amount":0.0,"bid":[[0,0.0],[0,0.0],[0,0.0],[0,0.0],[0,0.0]],"ask":[[0,0.0],[0,0.0],[0,0.0],[0,0.0],[0,0.0]],"date":"2022-03-01","time":"2022-03-01 00:00:00"}}}}
# {"morning_end":{
#     "opts":{"kind":"stock","frequency":86400,"codes":["sh600063","sh601456","sh000001"],"start_date":"2021-03-01","end_date":"2022-03-01"},"time":"2022-03-01T11:30:00"}}
# {"noon_start":{
#     "opts":{"kind":"stock","frequency":86400,"codes":["sh600063","sh601456","sh000001"],"start_date":"2021-03-01","end_date":"2022-03-01"},"time":"2022-03-01T13:00:00"}}
# {"noon_end":{
#     "opts":{"kind":"stock","frequency":86400,"codes":["sh600063","sh601456","sh000001"],"start_date":"2021-03-01","end_date":"2022-03-01"},"time":"2022-03-01T15:00:00"}}


class Strategy(BaseStrategy):
    """
    策略类接口，可产生信号:
    1. signal/[signal, ...] 取消委托/委托买/委托卖
    2. str/[str, ...]       订阅行情
    """

    def __init__(self):
        super().__init__()

        funcs = dict(
            on_open=self.on_open,
            on_close=self.on_close,
            on_quot=self.on_quot
        )

        self.funcs.update(funcs)

    def on_open(self, evt, payload) -> Optional[Union[Signal, str, List[Signal], List[str]]]:
        """
        开始事件回调
        :param evt: morning-start/noon-start
                    交易日早市开始/交易日午市开始
        :param payload:
        :return:
        """
        pass

    def on_close(self, evt, payload) -> Optional[Union[Signal, str, List[Signal], List[str]]]:
        """
        结束事件回调
        :param evt: morning_end/noon_end
                    交易日早市结束/交易日午市结束
        :param payload:
        :return:
        """
        pass

    def on_quot(self, evt, payload) -> Optional[Union[Signal, str, List[Signal], List[str]]]:
        """
        行情回调

        :param evt: quot
                    行情事件

        :param payload: 行情数据
        :return:
        """
        return None
