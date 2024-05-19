# frozen_string_literal: true

require 'sinatra'
require 'json'

puts "hello world"

get '/' do
  return 'pong'
end
