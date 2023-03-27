CREATE MIGRATION m1yl6splszb7ycf6vy4y43wjctkffyaaku2ldpwhxl6yct5sxxt7sq
    ONTO m1uyvctox6e7tkuljcc2o67m2irlr2mioywufrddz22mr6ybneqdaa
{
  ALTER TYPE default::Client {
      ALTER LINK address {
          SET MULTI;
      };
  };
  ALTER TYPE default::Client {
      ALTER LINK appliance {
          SET MULTI;
      };
  };
};
