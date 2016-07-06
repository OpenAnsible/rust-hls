Rust HTTP Live Stream Library
------------------------------------

:Date: 07/06 2016

介绍
--------

`HTTP Live Stream Library (HLS)` 是 `Apple` 开发的基于 `HTTP` 的串流规范，目前正在申请成为 `IETF` 组织的标准规范（当前为草案阶段）。

*该协议优点:*

1.  完全基于 HTTP 协议，意味着可以直接穿越防火墙，浏览器可以直接使用。
2.  视频分片机制有利于大规模的内容分发（这点，其它协议很难做到，比如 RTMP/RTSP ）
3.  Apple 移动端 只支持该方案（没有 `Flash` , 没有 `WebRTC` ），所以如果你想让 `Apple` 移动端的用户能用上你们的流服务， 那么选择 `HLS` 吧 :))

*该协议缺点:*
    
1.  延时高（纯粹就协议设计来讲，如果你架构够好，你完全可以使用 HLS 打造出延时低于 `RTMP` 的服务出来（比如 YouTuBe） ）


*哪些服务在使用该规范？:*
    
*   `Youtube Live/VOD <https://www.youtube.com/>`_ , 直播和点播场景均首选 `HLS` 方案
*   `TwitchTV <https://www.twitch.tv/>`_ , 之前使用的是 `RTMP` 方案，目前已经过渡到了 `HLS` 技术方案。


*哪些云服务支持该规范？:*

*   `七牛 <http://www.qiniu.com>`_ 
*   `腾讯云 <http://qcloud.com>`_
*   `UCloud <http://ucloud.cn>`_
*   `UpYun <https://www.upyun.com>`_
*   `Aliyun <https://www.aliyun.com/>`_


用例
--------

*   TODO


参考
---------

*   `HTTP Live Streaming <https://developer.apple.com/streaming/>`_ 
*   `draft-pantos-http-live-streaming <http://tools.ietf.org/html/draft-pantos-http-live-streaming>`_ , HTTP Live Streaming Internet - Draft


*JavaScript HTTP Live Stream Player:*

*   `hls.js <https://github.com/dailymotion/hls.js>`_ , 支持 `HLS` , 支持 `MPEG-2 Transport Stream` , `MP4` , `HLS`
*   `HTML5 Adaptive Streaming Player <https://bitmovin.com/demo/>`_ , 支持 `HTML5` , `HLS` , `DASH` , `MPEG-DASH`
