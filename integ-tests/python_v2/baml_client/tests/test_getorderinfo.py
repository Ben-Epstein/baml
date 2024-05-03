# This file is generated by the BAML compiler.
# Do not edit this file directly.
# Instead, edit the BAML files and recompile.

# ruff: noqa: E501,F401
# flake8: noqa: E501,F401
# pylint: disable=unused-import,line-too-long
# fmt: off

from ..__do_not_import.generated_baml_client import baml
from ..baml_types import Email, IGetOrderInfo, IGetOrderInfoStream, OrderInfo, OrderStatus
from baml_lib._impl.deserializer import Deserializer
from json import dumps
from pytest_baml.ipc_channel import BaseIPCChannel
from typing import Any


@baml.GetOrderInfo.test(stream=True)
async def test_interim_purple(GetOrderInfoImpl: IGetOrderInfoStream, baml_ipc_channel: BaseIPCChannel):
    def to_str(item: Any) -> str:
        if isinstance(item, str):
            return item
        return dumps(item)

    case = {""email"": {""subject"": "Your Amazon.com order of \"Wood Square Dowel Rods...\" has shipped!", ""body"": "Content-Type: text/plain; charset=utf-8
Content-Transfer-Encoding: 7bit

Amazon Shipping Confirmation
https://www.amazon.com?ie=UTF8&ref_=scr_home

____________________________________________________________________

Hi Samuel, your package will arrive:

Thursday, April 4

Track your package:
https://www.amazon.com/gp/your-account/ship-track?ie=UTF8&orderId=113-7540940-3785857&packageIndex=0&shipmentId=Gx7wk71F9&ref_=scr_pt_tp_t



On the way:
Wood Square Dowel Rods...
Order #113-7540940-3785857



An Amazon driver may contact you by text message or call you for help on the day of delivery.    

Ship to:
    Sam
    SEATTLE, WA

Shipment total:
$0.00
Rewards points applied


Return or replace items in Your orders
https://www.amazon.com/gp/css/order-history?ie=UTF8&ref_=scr_yo

Learn how to recycle your packaging at Amazon Second Chance(https://www.amazon.com/amsc?ref_=ascyorn).

", ""from"": "inov-8 <enquiries@inov-8.com>", ""from_address"": "\"Amazon.com\" <shipment-tracking@amazon.com>", }, }
    deserializer_email = Deserializer[Email](Email) # type: ignore
    email = deserializer_email.from_string(to_str(case["email"]))
    async with GetOrderInfoImpl(
        email=email
    ) as stream:
        async for response in stream.parsed_stream:
            baml_ipc_channel.send("partial_response", response.json())

        await stream.get_final_response()