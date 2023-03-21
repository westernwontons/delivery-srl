CREATE MIGRATION m1uyvctox6e7tkuljcc2o67m2irlr2mioywufrddz22mr6ybneqdaa
    ONTO m1wgzk43aph7g4sk37igny5pptebxbair45q5ajbilowyfmhhdpm6a
{
  ALTER TYPE default::Appliance {
      ALTER PROPERTY observations {
          RESET OPTIONALITY;
      };
  };
};
