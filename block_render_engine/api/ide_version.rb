Handler = Proc.new do |request, response|
  response.status = 200
  response['Content-Type'] = 'text/text; charset=utf-8'
  response.body = {
    dtr_core: Gem.loaded_specs['dtr_core'].version,
    block_render_engine: '0.1.0'
  }.to_json
end
