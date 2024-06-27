# frozen_string_literal: true

require 'spec_helper'

describe DTRToRust::Instruction::Divide do
  describe '#handle' do
    it 'returns the correct Rust code when var seen before' do
      instruction = DTRCore::Instruction.new('divide', %w[foo bar], 'foo', 0)
      expect(described_class.handle(instruction, 0, [], [], false, { 'foo' => [1] },
                                    {})).to eq('        foo = foo / bar;')
    end

    it 'returns the correct Rust code when var not seen before' do
      instruction = DTRCore::Instruction.new('divide', %w[foo bar], 'foo', 0)
      expect(described_class.handle(instruction, 0, [], [], false, {},
                                    {})).to eq('        let mut foo = foo / bar;')
    end
  end
end
