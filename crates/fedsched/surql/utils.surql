/*
UTILS
    the file contains helper functions
    this is the first surql to execute
    therefore functions defined here can be used anywhere
*/

DEFINE FUNCTION fn::unwrap($some: any) -> any {
    IF $some == NONE {
        THROW "Called unwrap on NONE";
    };
    IF type::is::string($some) AND count($some) == 0 {
        THROW "Called unwrap on an empty string";
    };
    IF type::is::object($some) AND count($some) == 0 {
        THROW "Called unwrap on an empty object";
    };
    IF type::is::array($some) AND count($some) == 0 {
        THROW "Called unwrap on an empty array";
    };

    RETURN $some;
};
DEFINE FUNCTION fn::assert_eq($lha: any, $rha: any) {
    IF $lha != $rha {
        THROW "Item `" + $lha + "` and `" + $rha + "` are not equal";
    }
};
DEFINE FUNCTION fn::assert_ne($lha: any, $rha: any) {
    IF $lha == $rha {
    THROW "Item `" + $lha + "` and `" + $rha + "` are equal, while forbidden to be so";
    }
};
