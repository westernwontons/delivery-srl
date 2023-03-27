CREATE MIGRATION m1yob45drimou4mzp4cc62tsl7cd5sbyjiannoj5kfurbbwmhzq6uq
    ONTO m1zi2vbqghnqwotanhbc527irctzehobjjyhhanczw544tbx6myofa
{
  ALTER SCALAR TYPE default::CustomerStatus EXTENDING enum<Active, Inactive>;
  ALTER TYPE default::Customer {
      ALTER PROPERTY status {
          SET default := (default::CustomerStatus.Active);
      };
  };
};
