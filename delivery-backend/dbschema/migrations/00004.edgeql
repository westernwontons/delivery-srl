CREATE MIGRATION m1g6av67r57wtsm4kuepy5u3sk5iosmvtys4jsavvhyuimr6gi3yka
    ONTO m1yl6splszb7ycf6vy4y43wjctkffyaaku2ldpwhxl6yct5sxxt7sq
{
  ALTER TYPE default::Client RENAME TO default::Customer;
};
