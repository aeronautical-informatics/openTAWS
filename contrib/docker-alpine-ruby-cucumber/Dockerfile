FROM ruby:alpine

RUN apk add build-base && gem install cucumber
ENTRYPOINT [ "cucumber", "--publish-quiet" ]
