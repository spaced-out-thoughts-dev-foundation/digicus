# frozen_string_literal: true

module DTRCore
  module Number
    MIN_U32 = 0
    MAX_U32 = (2**32) - 1

    MIN_U64 = 0
    MAX_U64 = (2**64) - 1

    MIN_U256 = 0
    MAX_U256 = (2**256) - 1

    MIN_I32 = -2**31
    MAX_I32 = (2**31) - 1

    MIN_I64 = -2**63
    MAX_I64 = (2**63) - 1

    MIN_I256 = -2**255
    MAX_I256 = (2**255) - 1
  end
end
