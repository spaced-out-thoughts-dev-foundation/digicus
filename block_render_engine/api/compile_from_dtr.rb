require 'dtr_core'
require_relative '../lib/block_render_engine_request_handler.rb'

Handler = Proc.new do |request, response|
  response.status = 200
  response['Content-Type'] = 'text/text; charset=utf-8'
  response.body = BlockRenderEngineRequestHandler.response_body(request)
end
