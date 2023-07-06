// SPDX-License-Identifier: MIT
pragma solidity >=0.4.0;

contract TestContract {
    uint256 nonce;

    event NonceEvent(
        uint256 nonce
    );

    constructor() {
        nonce = 0;
    }

    function increase() public {
        nonce += 1;

        emit NonceEvent(nonce);
    }

    function get() public view returns (uint) {
        return nonce;
    }
}