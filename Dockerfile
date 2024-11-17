FROM alpine:3.20.3


RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.ustc.edu.cn/g' /etc/apk/repositories && apk update && apk add --no-cache bash

WORKDIR /root/services/

COPY xray-rust .

CMD [ "./xray-rust" ]