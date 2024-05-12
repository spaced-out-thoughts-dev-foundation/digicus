class BlockRenderEngineRequestHandler
  SUCCESS_STATUS_CODE = 200
  NO_BODY_STATUS_CODE = 401
  FAILED_TO_COMPILE_STATUS_CODE = 402

  def initialize(request)
    @request = request
  end

  def self.response_body(request)
    new(request).response_body
  end

  def response_body
    return default_response unless @request.body && dtr?

    compile

    {
      received: {
        content: content,
        format: content_format,
      },
      contract_name: @contract_name,
      contract_state: @contract_state,
      contract_functions: @contract_functions,
      compilation_error: @compilation_error,
      status: status
    }.to_json
  end

  private

  def dtr?
    format == 'dtr'
  end

  def compile
     begin
      contract = DTRCore::Contract.from_dtr_raw(content)

      @contract_name = contract.name
      @contract_state = contract.state&.map do |s|
        { 
          name: s.name,
          type: s.type,
          initial_value: s.initial_value
        }.to_json
      end
      @contract_functions = contract.functions&.map do |f|
        { 
          name: f.name,
          instructions: f.instructions&.map { |x| x[:instruction] }.join(' ')
        }.to_json
      end

      @compilation_success = true
      @compilation_error = ''
    rescue StandardError => e
      @contract_name = "Unknown"
      @contract_state = []
      @contract_functions = []

      @compilation_error = e
    end
  end

  def status
   @compilation_success ? SUCCESS_STATUS_CODE : FAILED_TO_COMPILE_STATUS_CODE
  end

  def dtr_core_gem_version
    Gem.loaded_specs['dtr_core'].version
  end
  
  def content
    JSON.parse(@request.body)['content'].gsub('\n', "\n") || ''
  end
  
  def content_format
    JSON.parse(@request.body)['format'] || 'unknown'
  end

  def default_response
    { status: NO_BODY_STATUS_CODE }.to_json
  end
end
