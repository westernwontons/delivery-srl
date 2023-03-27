CREATE MIGRATION m15u25wnoxm3kbgavaj3wi3daqfsqcblvqdcncopg4zoqutmjsdveq
    ONTO m1wa6dt45yaelq25ssehm4ichplj44lhcm4foihv676wucbiolz6ca
{
  ALTER TYPE default::Appliance {
      ALTER PROPERTY expired {
          RESET default;
          USING ((.expiry_date > (.expiry_date - <std::duration>'3 days')));
      };
  };
  ALTER TYPE default::Appliance {
      DROP PROPERTY is_handled;
  };
  ALTER TYPE default::Appliance {
      ALTER PROPERTY last_updated {
          SET readonly := true;
      };
  };
  ALTER TYPE default::Customer {
      ALTER PROPERTY document_id {
          RENAME TO customer_id;
      };
  };
  ALTER TYPE default::Customer {
      ALTER LINK address {
          RESET OPTIONALITY;
      };
  };
  ALTER TYPE default::Customer {
      ALTER LINK appliance {
          RESET OPTIONALITY;
      };
  };
};
