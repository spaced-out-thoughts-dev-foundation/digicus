# frozen_string_literal: true

require 'sinatra'
require 'json'

puts "server started up"

get '/' do
  puts "pinged the server"
  return 'pong'
end

get '/home' do
  erb :home_page
end
