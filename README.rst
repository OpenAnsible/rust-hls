Rust HTTP Live Stream Library
------------------------------------

:Date: 07/06 2016

介绍
--------

`HTTP Live Stream Library (HLS)` 是 `Apple` 开发的基于 `HTTP` 的串流规范，目前正在申请成为 `IETF` 组织的标准规范（当前为草案阶段）。

*该协议优点:*

    1.  完全基于 `HTTP` 协议，意味着可以直接穿越防火墙，浏览器可以直接使用。
    2.  视频分片机制有利于大规模的内容分发（这点，其它协议很难做到，比如 `RTMP/RTSP` ）
    3.  `Apple` 移动端 只支持该方案（没有 `Flash` , 没有 `WebRTC` ），
        所以如果你想让 `Apple` 移动端的用户能用上你们的流服务，那么选择 `HLS` 吧 :))

*该协议缺点:*
    
    1.  延时高（纯粹就协议设计来讲，如果你架构够好，你完全可以使用 `HLS` 打造出延时低于 `RTMP` 的服务出来, 比如 `YouTuBe` ）
    2.  单向流 

*哪些服务在使用该规范？:*
    
    *   `Youtube  <https://www.youtube.com/>`_ , 直播和点播场景均首选 `HLS` 方案
    *   `TwitchTV <https://www.twitch.tv/>`_ , 之前使用的是 `RTMP` 方案，目前已经过渡到了 `HLS` 技术方案。


*哪些云服务支持该规范？:*

    *   `QiNiu  <http://www.qiniu.com>`_ 
    *   `QCloud <http://qcloud.com>`_
    *   `UCloud <http://ucloud.cn>`_
    *   `UpYun  <https://www.upyun.com>`_
    *   `Aliyun <https://www.aliyun.com/>`_


原理简介
-----------

直播模式
^^^^^^^^^^

*Server:*

0.  初始化视频片索引文件（比如 `live.m3u8` ）。
1.  从视频源收集 `视频帧` (AVFrame), 然后对 `视频帧` 进行编码（比如 `H264` ）。
2.  收集固定时长（一般是5秒）的已编码的 `视频帧` 数据, 合并成一片 `MPEG-TS` 数据。
3.  将合并完成的 `MPEG-TS` 片数据存储至 静态文件服务器（或者CDN）。
4.  将已存储的 `MPEG-TS` 片数据公网访问路径(比如: http://live.com/object/d1162b6b627b8726f91e3d59feaf5db188e95219.ts ) 更新进 `M3U8` 索引文件。

*Client:*

0.  下载视频流索引文件 `M3U8`，获得 `MPEG-TS` 数据片。
1.  按照索引顺序下载 `MPEG-TS` 数据片，解码并播放（非 `Apple` 浏览器，可能需要 `JavaScript` 来做软解码）。
2.  索引播放完毕，重新下载 `M3U8` 看是否更新了索引文件，如果更新，则继续重复上述步骤直至索引文件标示 `结束` 。


点播模式
^^^^^^^^^^

这个模式没什么好讲的，对视频文件进行解码操作，得到 `视频帧`，然后后续操作跟 `直播` 流程一样， 
只是对 索引文件 的处理方式不太一样。



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
