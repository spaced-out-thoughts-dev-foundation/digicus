# frozen_string_literal: true

require './spec/spec_helper'

RSpec.describe DTRCore::TypeValidator do
  context 'when the type is I32 and type too big' do
    it 'raises an error' do
      expect do
        described_class.new('I32', (DTRCore::Number::MAX_I32 + 1).to_s).validate_then_coerce_initial_value!
      end.to raise_error(/Invalid initial value for type I32. Out of range./)
    end
  end

  context 'when the type is I32 and type too small' do
    it 'raises an error' do
      expect do
        described_class.new('I32', (DTRCore::Number::MIN_I32 - 1).to_s).validate_then_coerce_initial_value!
      end.to raise_error(/Invalid initial value for type I32. Out of range./)
    end
  end

  context 'when the type is I32 and type is not a number' do
    it 'raises an error' do
      expect do
        described_class.new('I32', 'Apple').validate_then_coerce_initial_value!
      end.to raise_error(/Invalid initial value for type. Wrong type./)
    end
  end

  context 'when the type is I64 and type too big' do
    it 'raises an error' do
      expect do
        described_class.new('I64', (DTRCore::Number::MAX_I64 + 1).to_s).validate_then_coerce_initial_value!
      end.to raise_error(/Invalid initial value for type I64. Out of range./)
    end
  end

  context 'when the type is I64 and type too small' do
    it 'raises an error' do
      expect do
        described_class.new('I64', (DTRCore::Number::MIN_I64 - 1).to_s).validate_then_coerce_initial_value!
      end.to raise_error(/Invalid initial value for type I64. Out of range./)
    end
  end

  context 'when the type is I64 and type is not a number' do
    it 'raises an error' do
      expect do
        described_class.new('I64', 'Apple').validate_then_coerce_initial_value!
      end.to raise_error(/Invalid initial value for type. Wrong type./)
    end
  end

  context 'when the type is I256 and type too big' do
    it 'raises an error' do
      expect do
        described_class.new('I256', (DTRCore::Number::MAX_I256 + 1).to_s).validate_then_coerce_initial_value!
      end.to raise_error(/Invalid initial value for type I256. Out of range./)
    end
  end

  context 'when the type is I256 and type too small' do
    it 'raises an error' do
      expect do
        described_class.new('I256', (DTRCore::Number::MIN_I256 - 1).to_s).validate_then_coerce_initial_value!
      end.to raise_error(/Invalid initial value for type I256. Out of range./)
    end
  end

  context 'when the type is I256 and type is not a number' do
    it 'raises an error' do
      expect do
        described_class.new('I256', 'A 9223372036854775807').validate_then_coerce_initial_value!
      end.to raise_error(/Invalid initial value for type. Wrong type./)
    end
  end

  context 'when the type is U32 and type too big' do
    it 'raises an error' do
      expect do
        described_class.new('U32', (DTRCore::Number::MAX_U32 + 1).to_s).validate_then_coerce_initial_value!
      end.to raise_error(/Invalid initial value for type U32. Out of range./)
    end
  end

  context 'when the type is U32 and type too small' do
    it 'raises an error' do
      expect do
        described_class.new('U32', (DTRCore::Number::MIN_U32 - 1).to_s).validate_then_coerce_initial_value!
      end.to raise_error(/Invalid initial value for type U32. Out of range./)
    end
  end

  context 'when the type is U32 and type is not a number' do
    it 'raises an error' do
      expect do
        described_class.new('U32', 'Apple').validate_then_coerce_initial_value!
      end.to raise_error(/Invalid initial value for type. Wrong type./)
    end
  end

  context 'when the type is U64 and type too big' do
    it 'raises an error' do
      expect do
        described_class.new('U64', (DTRCore::Number::MAX_U64 + 1).to_s).validate_then_coerce_initial_value!
      end.to raise_error(/Invalid initial value for type U64. Out of range./)
    end
  end

  context 'when the type is U64 and type too small' do
    it 'raises an error' do
      expect do
        described_class.new('U64', (DTRCore::Number::MIN_U64 - 1).to_s).validate_then_coerce_initial_value!
      end.to raise_error(/Invalid initial value for type U64. Out of range./)
    end
  end

  context 'when the type is U64 and type is not a number' do
    it 'raises an error' do
      expect do
        described_class.new('U64', 'Apple').validate_then_coerce_initial_value!
      end.to raise_error(/Invalid initial value for type. Wrong type./)
    end
  end

  context 'when the type is U256 and type too big' do
    it 'raises an error' do
      expect do
        described_class.new('U256', (DTRCore::Number::MAX_U256 + 1).to_s).validate_then_coerce_initial_value!
      end.to raise_error(/Invalid initial value for type U256. Out of range./)
    end
  end

  context 'when the type is U256 and type too small' do
    it 'raises an error' do
      expect do
        described_class.new('U256', (DTRCore::Number::MIN_U256 - 1).to_s).validate_then_coerce_initial_value!
      end.to raise_error(/Invalid initial value for type U256. Out of range./)
    end
  end

  context 'when the type is U256 and type is not a number' do
    it 'raises an error' do
      expect do
        described_class.new('U256', 'Apple').validate_then_coerce_initial_value!
      end.to raise_error(/Invalid initial value for type. Wrong type./)
    end
  end
end
