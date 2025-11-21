FROM elixir:1.15-alpine

RUN apk add --no-cache git build-base

RUN mix local.hex --force && \
    mix local.rebar --force

COPY . .

RUN mix do deps.get, deps.compile

CMD ["mix", "phx.server"]

