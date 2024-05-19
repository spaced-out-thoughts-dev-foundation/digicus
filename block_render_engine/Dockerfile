# Dockerfile

FROM ruby:3.2.2

WORKDIR /code
COPY . /code
RUN bundle install

EXPOSE 8888

CMD ["bundle", "exec", "puma", "-C", "puma.rb"]