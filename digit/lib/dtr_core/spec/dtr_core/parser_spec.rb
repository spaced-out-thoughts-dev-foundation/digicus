# frozen_string_literal: true

require './spec/spec_helper'

RSpec.describe DTRCore::Parser do
  context 'when file does not exist' do
    it 'raises an error' do
      expect { described_class.parse('DTR') }.to raise_error(/Unable to find file: DTR./)
    end
  end

  context 'when invalid DTR' do
    context 'when contract name section missing' do
      it 'raises an error' do
        expect do
          described_class.parse('./spec/test_dtr_files/missing_contract_name_section.dtr')
        end.to raise_error(/Missing contract name./)
      end
    end

    context 'when state section is empty' do
      it 'raises an error' do
        expect do
          described_class.parse('./spec/test_dtr_files/state_section_no_definitions_empty_error.dtr')
        end.to raise_error(/Empty state section./)
      end
    end

    context 'when state section includes a definition with missing type' do
      it 'raises an error' do
        expect do
          described_class.parse('./spec/test_dtr_files/state_section_missing_type_name.dtr')
        end.to raise_error(/Missing Type Name./)
      end
    end

    context 'when state section includes a definition with missing initial value' do
      it 'raises an error' do
        expect do
          described_class.parse('./spec/test_dtr_files/state_section_missing_initial_value.dtr')
        end.to raise_error(/Missing Initial Value./)
      end
    end

    context 'when state section includes an unrecognized type' do
      it 'parses the contract name section but the state section is nil' do
        expect do
          described_class.parse('./spec/test_dtr_files/state_section_invalid_type_name.dtr')
        end.to raise_error(/Missing Invalid Type Name./)
      end
    end
  end

  context 'when valid DTR' do
    context 'when only contract name section is present' do
      it 'parses the contract name section' do
        contract = described_class.parse('./spec/test_dtr_files/contract_name_section_only.dtr')

        expect(contract.name).to eq('CONTRACT_NAME')

        # empty contract so these optional sections are nil
        expect(contract.state).to be_nil
        expect(contract.functions).to be_nil
      end

      it 'parses the name of the contract even when it is weird' do
        contract = described_class.parse('./spec/test_dtr_files/contract_name_section_only_weird_name.dtr')

        expect(contract.name).to eq('CONTRACT_NAME is foo 123')

        # empty contract so these optional sections are nil
        expect(contract.state).to be_nil
        expect(contract.functions).to be_nil
      end
    end

    context 'when contract name and state sections are present' do
      it 'parses the contract name and state sections' do
        contract = described_class.parse('./spec/test_dtr_files/contract_and_state_only_swapped_order.dtr')

        expect(contract.name).to eq('CONTRACT_NAME')

        expect(contract.state).to contain_exactly(DTRCore::State.new('STATE_DEFINITION_1', 'I32', 22))

        # empty contract so these optional sections are nil
        expect(contract.functions).to be_nil
      end
    end

    context 'when contract name and function sections are present' do
      let(:hello_function) do
        DTRCore::Function.new('hello', [
                                { name: 'to', type_name: 'Symbol' }
                              ], 'Symbol', [
                                { instruction: 'AddSymbols',
                                  inputs: %w[Hello to], assign: 'HelloToResult' },
                                { instruction: 'Return',
                                  inputs: ['HelloToResult'], assign: nil }
                              ])
      end

      it 'parses the contract name and function sections' do
        contract = described_class.parse('./spec/test_dtr_files/hello_world_simple.dtr')

        expect(contract.name).to eq('HelloContract')

        # empty contract so these optional sections are nil
        expect(contract.state).to be_nil

        expect(contract.functions).to contain_exactly(hello_function)
      end
    end

    context 'when contract name, state, and function sections are present' do
      let(:hello_function) do
        DTRCore::Function.new('hello', [
                                { name: 'to', type_name: 'Symbol' },
                                { name: 'from', type_name: 'I32' }
                              ], 'Symbol', [
                                { instruction: 'AddSymbols',
                                  inputs: %w[Hello to], assign: 'HelloToResult' },
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
        contract = described_class.parse('./spec/test_dtr_files/multi_function_with_state_and_name_contract.dtr')

        expect(contract.name).to eq('MultiFunctionContract')
        expect(contract.state).to match_array(state_definitions)
        expect(contract.functions).to contain_exactly(hello_function, world_function)
      end
    end
  end
end
