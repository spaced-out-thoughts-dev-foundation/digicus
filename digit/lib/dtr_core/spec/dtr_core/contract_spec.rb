# frozen_string_literal: true

require './spec/spec_helper'

RSpec.describe DTRCore::Contract do
  context 'when passing filepath' do
    let(:hello_function) do
      DTRCore::Function.new('hello', [
                              { name: 'to', type_name: 'Symbol' },
                              { name: 'from', type_name: 'I32' }
                            ], 'Symbol', [
                              { instruction: 'AddSymbols',
                                inputs: ['"Hello"', 'to'], assign: 'HelloToResult' },
                              { instruction: 'Return',
                                inputs: ['HelloToResult'], assign: nil }
                            ])
    end

    let(:world_function) do
      DTRCore::Function.new('world', [], 'Symbol', [
                              { instruction: 'Return', inputs: nil, assign: 'ReturnValue' }
                            ])
    end

    let(:state_definitions) do
      [
        DTRCore::State.new('STATE_DEFINITION_1', 'I32', 22),
        DTRCore::State.new('STATE_DEFINITION_2', 'Symbol', 'Hello World'),
        DTRCore::State.new('STATE_DEFINITION_3', 'I256', -1234)
      ]
    end

    it 'parses the contract name, state, and function sections' do
      contract = described_class.from_dtr('./spec/test_dtr_files/multi_function_with_state_and_name_contract.dtr')

      expect(contract.name).to eq('MultiFunctionContract')
      expect(contract.state).to match_array(state_definitions)
      expect(contract.functions).to contain_exactly(hello_function, world_function)
    end
  end

  context 'when passing content directly' do
    let(:hello_function) do
      DTRCore::Function.new('hello', [
                              { name: 'to', type_name: 'Symbol' },
                              { name: 'from', type_name: 'I32' }
                            ], 'Symbol', [
                              { instruction: 'AddSymbols',
                                inputs: ['"Hello"', 'to'], assign: 'HelloToResult' },
                              { instruction: 'Return',
                                inputs: ['HelloToResult'], assign: nil }
                            ])
    end

    let(:world_function) do
      DTRCore::Function.new('world', [], 'Symbol', [
                              { instruction: 'Return', inputs: nil, assign: 'ReturnValue' }
                            ])
    end

    let(:state_definitions) do
      [
        DTRCore::State.new('STATE_DEFINITION_1', 'I32', 22),
        DTRCore::State.new('STATE_DEFINITION_2', 'Symbol', 'Hello World'),
        DTRCore::State.new('STATE_DEFINITION_3', 'I256', -1234)
      ]
    end

    it 'parses the contract name, state, and function sections' do
      content = File.read('./spec/test_dtr_files/multi_function_with_state_and_name_contract.dtr')

      contract = described_class.from_dtr_raw(content)

      expect(contract.name).to eq('MultiFunctionContract')
      expect(contract.state).to match_array(state_definitions)
      expect(contract.functions).to contain_exactly(hello_function, world_function)
    end
  end
end
