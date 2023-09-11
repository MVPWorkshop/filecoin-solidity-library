// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.17;

import {Test} from "forge-std/Test.sol";
import {FilAddresses} from "contracts/v0.8/utils/FilAddresses.sol";
import {CommonTypes} from "contracts/v0.8/types/CommonTypes.sol";

contract FilAddressesTest is Test {
    error InvalidAddress();

    function testFuzz_toEthAddressInvalidFirstByte(
       address addr,
       bytes1 firstByte
    ) public {
        vm.assume(firstByte != hex"04");
        CommonTypes.FilAddress memory filAddress = CommonTypes.FilAddress(
          abi.encodePacked(firstByte, hex"0a", addr)
        );

        vm.expectRevert(InvalidAddress.selector);
        FilAddresses.toEthAddress(filAddress);
    }

    function testFuzz_toEthAddressInvalidSecondByte(
        address addr,
        bytes1 secondByte
    ) public {
        vm.assume(secondByte != hex"0a");
        CommonTypes.FilAddress memory filAddress = CommonTypes.FilAddress(
          abi.encodePacked(hex"04", secondByte, addr)
        );

        vm.expectRevert(InvalidAddress.selector);
        FilAddresses.toEthAddress(filAddress);
    }

    function testFuzz_toEthAddressInvalidBytesLength(
        address addr,
        bytes1 endByte
    ) public {
        CommonTypes.FilAddress memory filAddress = CommonTypes.FilAddress(
          abi.encodePacked("040b", addr, endByte)
        );

        vm.expectRevert(InvalidAddress.selector);
        FilAddresses.toEthAddress(filAddress);
    }

    function testFuzz_toEthAddress(address addr) public {
        bytes memory addrBytes = abi.encodePacked(addr);
        bytes memory filAddressBytes = abi.encodePacked(hex"040a", addrBytes);
        CommonTypes.FilAddress memory filAddress = CommonTypes.FilAddress(filAddressBytes);

        address ethAddress = FilAddresses.toEthAddress(filAddress);

        assertEq(addr, ethAddress);
    }

    function testFuzz_fromEthAddress(address addr) public {
        bytes memory addrBytes = abi.encodePacked(addr);
        bytes memory filAddressBytes = abi.encodePacked(hex"040a", addrBytes);

        CommonTypes.FilAddress memory filAddress = FilAddresses.fromEthAddress(addr);

        assertEq(filAddressBytes, filAddress.data);
    }
}
