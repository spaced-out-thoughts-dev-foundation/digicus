# Dockerfile

FROM ruby:3.2.2

WORKDIR /code
COPY . /code
RUN bundle install

EXPOSE 8080

CMD ["bundle", "exec", "puma", "-C", "puma.rb"]