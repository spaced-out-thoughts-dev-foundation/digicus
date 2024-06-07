# frozen_string_literal: true

require 'spec_helper'

describe DTRToRust::Instruction::CreateList do
  describe '#handle' do
    it 'returns the correct Rust code for a list with no elements' do
      instruction = {
        instruction: 'create_list',
        inputs: [],
        assign: 'some_list',
        scope: 0
      }

      expect(described_class.handle(instruction)).to eq('        let some_list = vec![];')
    end

    it 'returns the correct Rust code for a list with multiple elements' do
      instruction = {
        instruction: 'create_list',
        inputs: ['env', '"thing_1"', 'count', '10'],
        assign: 'some_list',
        scope: 0
      }

      expect(described_class.handle(instruction)).to eq('        let some_list = vec![&env, symbol_short!("thing_1"), count, 10];')
    end
  end
end
