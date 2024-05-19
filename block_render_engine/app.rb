# frozen_string_literal: true

require 'sinatra'
require 'json'

puts "server started up"

get '/' do
  puts "pinged the server"
  return 'pong'
end
