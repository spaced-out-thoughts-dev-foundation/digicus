# frozen_string_literal: true

require 'spec_helper'

describe DTRToRust::Instruction::Print do
  describe '#handle' do
    it 'returns the correct Rust code' do
      instruction = DTRCore::Instruction.new('print', ['env', '"count: {}"', 'count'], nil, 0)

      expect(described_class.handle(instruction, [], [], false)).to eq('        log!(&env, "count: {}", count);')
    end
  end
end
