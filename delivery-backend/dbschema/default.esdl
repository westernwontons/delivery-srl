module default {
	scalar type OperationPerformed extending enum<VTP, INT, PIF, RGAZ, VGAZ>; # operatia_efect
	scalar type CustomerStatus extending enum<Active, Inactive>;
	
	type Address {
		required property county -> str; # judet
		required property street -> str; # strada
		required property number -> str; # nr
		required property additional -> str; # bl_sc_ap
	}

	type Appliance {
		required property manufacturer -> str; # producator
		required property year_of_manufacture -> str; # nr_fabr_an_fabr
		required property model -> str; # model
		required property type -> str; # tip
		required property warranty -> datetime; # garantia
		required property operation_performed -> OperationPerformed; # operatia_efect
		required property number -> str; # numar
		required property date -> datetime; # data
		required property expiration_date -> datetime; # scadenta
		property observations -> str; # observatii
		required property last_updated -> datetime {
			default := datetime_current();
		};
		required property expired := .expiration_date > (.expiration_date - <duration>"3 days");
	}

	type Customer {
		required property customer_id -> str; # dosar
		required property name -> str; # destinator_utilizator
		required property status -> CustomerStatus {
			default := CustomerStatus.Active
		}
		multi link address -> Address; # adresa
		multi link appliance -> Appliance;
		index on (.customer_id);
	}
}
