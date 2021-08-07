CREATE TABLE "actor" (
    "actor_id" serial NOT NULL,
    "first_name" varchar(45) NOT NULL,
    "last_name" varchar(45) NOT NULL,
    "last_update" timestamp(6) NOT NULL DEFAULT now(),
    CONSTRAINT "actor_pkey" PRIMARY KEY ("actor_id")
);
CREATE TABLE "film" (
    "film_id" serial NOT NULL,
    "title" varchar(255) NOT NULL,
    "description" text,
    "release_year" integer,
    "language_id" smallint NOT NULL,
    "original_language_id" smallint,
    "rental_duration" smallint NOT NULL DEFAULT 3,
    "rental_rate" decimal(4, 2) NOT NULL DEFAULT 4.99,
    "length" smallint,
    "replacement_cost" decimal(5, 2) NOT NULL DEFAULT 19.99,
    "rating" USER - DEFINED DEFAULT 'G'::mpaa_rating,
    "last_update" timestamp(6) NOT NULL DEFAULT now(),
    "special_features" array,
    "fulltext" tsvector NOT NULL,
    CONSTRAINT "film_pkey" PRIMARY KEY ("film_id"),
    CONSTRAINT "film_language_id_fkey" FOREIGN KEY ("language_id") REFERENCES "language" ("language_id") ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT "film_original_language_id_fkey" FOREIGN KEY ("original_language_id") REFERENCES "language" ("language_id") ON DELETE RESTRICT ON UPDATE CASCADE
);
CREATE TABLE "payment_p2007_02" (
    "payment_id" serial NOT NULL,
    "customer_id" smallint NOT NULL,
    "staff_id" smallint NOT NULL,
    "rental_id" integer NOT NULL,
    "amount" decimal(5, 2) NOT NULL,
    "payment_date" timestamp(6) NOT NULL,
    CONSTRAINT "payment_p2007_02_customer_id_fkey" FOREIGN KEY ("customer_id") REFERENCES "customer" ("customer_id") ON DELETE NO ACTION ON UPDATE NO ACTION,
    CONSTRAINT "payment_p2007_02_rental_id_fkey" FOREIGN KEY ("rental_id") REFERENCES "rental" ("rental_id") ON DELETE NO ACTION ON UPDATE NO ACTION,
    CONSTRAINT "payment_p2007_02_staff_id_fkey" FOREIGN KEY ("staff_id") REFERENCES "staff" ("staff_id") ON DELETE NO ACTION ON UPDATE NO ACTION
);
CREATE TABLE "payment_p2007_03" (
    "payment_id" serial NOT NULL,
    "customer_id" smallint NOT NULL,
    "staff_id" smallint NOT NULL,
    "rental_id" integer NOT NULL,
    "amount" decimal(5, 2) NOT NULL,
    "payment_date" timestamp(6) NOT NULL,
    CONSTRAINT "payment_p2007_03_customer_id_fkey" FOREIGN KEY ("customer_id") REFERENCES "customer" ("customer_id") ON DELETE NO ACTION ON UPDATE NO ACTION,
    CONSTRAINT "payment_p2007_03_rental_id_fkey" FOREIGN KEY ("rental_id") REFERENCES "rental" ("rental_id") ON DELETE NO ACTION ON UPDATE NO ACTION,
    CONSTRAINT "payment_p2007_03_staff_id_fkey" FOREIGN KEY ("staff_id") REFERENCES "staff" ("staff_id") ON DELETE NO ACTION ON UPDATE NO ACTION
);
CREATE TABLE "payment_p2007_04" (
    "payment_id" serial NOT NULL,
    "customer_id" smallint NOT NULL,
    "staff_id" smallint NOT NULL,
    "rental_id" integer NOT NULL,
    "amount" decimal(5, 2) NOT NULL,
    "payment_date" timestamp(6) NOT NULL,
    CONSTRAINT "payment_p2007_04_customer_id_fkey" FOREIGN KEY ("customer_id") REFERENCES "customer" ("customer_id") ON DELETE NO ACTION ON UPDATE NO ACTION,
    CONSTRAINT "payment_p2007_04_rental_id_fkey" FOREIGN KEY ("rental_id") REFERENCES "rental" ("rental_id") ON DELETE NO ACTION ON UPDATE NO ACTION,
    CONSTRAINT "payment_p2007_04_staff_id_fkey" FOREIGN KEY ("staff_id") REFERENCES "staff" ("staff_id") ON DELETE NO ACTION ON UPDATE NO ACTION
);
CREATE TABLE "payment_p2007_05" (
    "payment_id" serial NOT NULL,
    "customer_id" smallint NOT NULL,
    "staff_id" smallint NOT NULL,
    "rental_id" integer NOT NULL,
    "amount" decimal(5, 2) NOT NULL,
    "payment_date" timestamp(6) NOT NULL,
    CONSTRAINT "payment_p2007_05_customer_id_fkey" FOREIGN KEY ("customer_id") REFERENCES "customer" ("customer_id") ON DELETE NO ACTION ON UPDATE NO ACTION,
    CONSTRAINT "payment_p2007_05_rental_id_fkey" FOREIGN KEY ("rental_id") REFERENCES "rental" ("rental_id") ON DELETE NO ACTION ON UPDATE NO ACTION,
    CONSTRAINT "payment_p2007_05_staff_id_fkey" FOREIGN KEY ("staff_id") REFERENCES "staff" ("staff_id") ON DELETE NO ACTION ON UPDATE NO ACTION
);
CREATE TABLE "payment_p2007_06" (
    "payment_id" serial NOT NULL,
    "customer_id" smallint NOT NULL,
    "staff_id" smallint NOT NULL,
    "rental_id" integer NOT NULL,
    "amount" decimal(5, 2) NOT NULL,
    "payment_date" timestamp(6) NOT NULL,
    CONSTRAINT "payment_p2007_06_customer_id_fkey" FOREIGN KEY ("customer_id") REFERENCES "customer" ("customer_id") ON DELETE NO ACTION ON UPDATE NO ACTION,
    CONSTRAINT "payment_p2007_06_rental_id_fkey" FOREIGN KEY ("rental_id") REFERENCES "rental" ("rental_id") ON DELETE NO ACTION ON UPDATE NO ACTION,
    CONSTRAINT "payment_p2007_06_staff_id_fkey" FOREIGN KEY ("staff_id") REFERENCES "staff" ("staff_id") ON DELETE NO ACTION ON UPDATE NO ACTION
);
CREATE TABLE "payment_p2007_01" (
    "payment_id" serial NOT NULL,
    "customer_id" smallint NOT NULL,
    "staff_id" smallint NOT NULL,
    "rental_id" integer NOT NULL,
    "amount" decimal(5, 2) NOT NULL,
    "payment_date" timestamp(6) NOT NULL,
    CONSTRAINT "payment_p2007_01_customer_id_fkey" FOREIGN KEY ("customer_id") REFERENCES "customer" ("customer_id") ON DELETE NO ACTION ON UPDATE NO ACTION,
    CONSTRAINT "payment_p2007_01_rental_id_fkey" FOREIGN KEY ("rental_id") REFERENCES "rental" ("rental_id") ON DELETE NO ACTION ON UPDATE NO ACTION,
    CONSTRAINT "payment_p2007_01_staff_id_fkey" FOREIGN KEY ("staff_id") REFERENCES "staff" ("staff_id") ON DELETE NO ACTION ON UPDATE NO ACTION
);
CREATE TABLE "address" (
    "address_id" serial NOT NULL,
    "address" varchar(50) NOT NULL,
    "address2" varchar(50),
    "district" varchar(20) NOT NULL,
    "city_id" smallint NOT NULL,
    "postal_code" varchar(10),
    "phone" varchar(20) NOT NULL,
    "last_update" timestamp(6) NOT NULL DEFAULT now(),
    CONSTRAINT "address_pkey" PRIMARY KEY ("address_id"),
    CONSTRAINT "address_city_id_fkey" FOREIGN KEY ("city_id") REFERENCES "city" ("city_id") ON DELETE RESTRICT ON UPDATE CASCADE
);
CREATE TABLE "category" (
    "category_id" serial NOT NULL,
    "name" varchar(25) NOT NULL,
    "last_update" timestamp(6) NOT NULL DEFAULT now(),
    CONSTRAINT "category_pkey" PRIMARY KEY ("category_id")
);
CREATE TABLE "city" (
    "city_id" serial NOT NULL,
    "city" varchar(50) NOT NULL,
    "country_id" smallint NOT NULL,
    "last_update" timestamp(6) NOT NULL DEFAULT now(),
    CONSTRAINT "city_pkey" PRIMARY KEY ("city_id"),
    CONSTRAINT "city_country_id_fkey" FOREIGN KEY ("country_id") REFERENCES "country" ("country_id") ON DELETE RESTRICT ON UPDATE CASCADE
);
CREATE TABLE "country" (
    "country_id" serial NOT NULL,
    "country" varchar(50) NOT NULL,
    "last_update" timestamp(6) NOT NULL DEFAULT now(),
    CONSTRAINT "country_pkey" PRIMARY KEY ("country_id")
);
CREATE TABLE "customer" (
    "customer_id" serial NOT NULL,
    "store_id" smallint NOT NULL,
    "first_name" varchar(45) NOT NULL,
    "last_name" varchar(45) NOT NULL,
    "email" varchar(50),
    "address_id" smallint NOT NULL,
    "activebool" bool NOT NULL DEFAULT true,
    "create_date" date NOT NULL DEFAULT ('now'::text)::date,
    "last_update" timestamp(6) DEFAULT now(),
    "active" integer,
    CONSTRAINT "customer_pkey" PRIMARY KEY ("customer_id"),
    CONSTRAINT "customer_address_id_fkey" FOREIGN KEY ("address_id") REFERENCES "address" ("address_id") ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT "customer_store_id_fkey" FOREIGN KEY ("store_id") REFERENCES "store" ("store_id") ON DELETE RESTRICT ON UPDATE CASCADE
);
CREATE TABLE "film_actor" (
    "actor_id" smallint NOT NULL,
    "film_id" smallint NOT NULL,
    "last_update" timestamp(6) NOT NULL DEFAULT now(),
    CONSTRAINT "film_actor_pkey" PRIMARY KEY ("actor_id", "film_id"),
    CONSTRAINT "film_actor_actor_id_fkey" FOREIGN KEY ("actor_id") REFERENCES "actor" ("actor_id") ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT "film_actor_film_id_fkey" FOREIGN KEY ("film_id") REFERENCES "film" ("film_id") ON DELETE RESTRICT ON UPDATE CASCADE
);
CREATE TABLE "film_category" (
    "film_id" smallint NOT NULL,
    "category_id" smallint NOT NULL,
    "last_update" timestamp(6) NOT NULL DEFAULT now(),
    CONSTRAINT "film_category_pkey" PRIMARY KEY ("film_id", "category_id"),
    CONSTRAINT "film_category_category_id_fkey" FOREIGN KEY ("category_id") REFERENCES "category" ("category_id") ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT "film_category_film_id_fkey" FOREIGN KEY ("film_id") REFERENCES "film" ("film_id") ON DELETE RESTRICT ON UPDATE CASCADE
);
CREATE TABLE "inventory" (
    "inventory_id" serial NOT NULL,
    "film_id" smallint NOT NULL,
    "store_id" smallint NOT NULL,
    "last_update" timestamp(6) NOT NULL DEFAULT now(),
    CONSTRAINT "inventory_pkey" PRIMARY KEY ("inventory_id"),
    CONSTRAINT "inventory_film_id_fkey" FOREIGN KEY ("film_id") REFERENCES "film" ("film_id") ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT "inventory_store_id_fkey" FOREIGN KEY ("store_id") REFERENCES "store" ("store_id") ON DELETE RESTRICT ON UPDATE CASCADE
);
CREATE TABLE "language" (
    "language_id" serial NOT NULL,
    "name" char(20) NOT NULL,
    "last_update" timestamp(6) NOT NULL DEFAULT now(),
    CONSTRAINT "language_pkey" PRIMARY KEY ("language_id")
);
CREATE TABLE "rental" (
    "rental_id" serial NOT NULL,
    "rental_date" timestamp(6) NOT NULL,
    "inventory_id" integer NOT NULL,
    "customer_id" smallint NOT NULL,
    "return_date" timestamp(6),
    "staff_id" smallint NOT NULL,
    "last_update" timestamp(6) NOT NULL DEFAULT now(),
    CONSTRAINT "rental_pkey" PRIMARY KEY ("rental_id"),
    CONSTRAINT "rental_customer_id_fkey" FOREIGN KEY ("customer_id") REFERENCES "customer" ("customer_id") ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT "rental_inventory_id_fkey" FOREIGN KEY ("inventory_id") REFERENCES "inventory" ("inventory_id") ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT "rental_staff_id_fkey" FOREIGN KEY ("staff_id") REFERENCES "staff" ("staff_id") ON DELETE RESTRICT ON UPDATE CASCADE
);
CREATE TABLE "staff" (
    "staff_id" serial NOT NULL,
    "first_name" varchar(45) NOT NULL,
    "last_name" varchar(45) NOT NULL,
    "address_id" smallint NOT NULL,
    "email" varchar(50),
    "store_id" smallint NOT NULL,
    "active" bool NOT NULL DEFAULT true,
    "username" varchar(16) NOT NULL,
    "password" varchar(40),
    "last_update" timestamp(6) NOT NULL DEFAULT now(),
    "picture" bytea,
    CONSTRAINT "staff_pkey" PRIMARY KEY ("staff_id"),
    CONSTRAINT "staff_address_id_fkey" FOREIGN KEY ("address_id") REFERENCES "address" ("address_id") ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT "staff_store_id_fkey" FOREIGN KEY ("store_id") REFERENCES "store" ("store_id") ON DELETE NO ACTION ON UPDATE NO ACTION
);
CREATE TABLE "store" (
    "store_id" serial NOT NULL,
    "manager_staff_id" smallint NOT NULL,
    "address_id" smallint NOT NULL,
    "last_update" timestamp(6) NOT NULL DEFAULT now(),
    CONSTRAINT "store_pkey" PRIMARY KEY ("store_id"),
    CONSTRAINT "store_address_id_fkey" FOREIGN KEY ("address_id") REFERENCES "address" ("address_id") ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT "store_manager_staff_id_fkey" FOREIGN KEY ("manager_staff_id") REFERENCES "staff" ("staff_id") ON DELETE RESTRICT ON UPDATE CASCADE
);
CREATE TABLE "payment" (
    "payment_id" serial NOT NULL,
    "customer_id" smallint NOT NULL,
    "staff_id" smallint NOT NULL,
    "rental_id" integer NOT NULL,
    "amount" decimal(5, 2) NOT NULL,
    "payment_date" timestamp(6) NOT NULL,
    CONSTRAINT "payment_pkey" PRIMARY KEY ("payment_id"),
    CONSTRAINT "payment_customer_id_fkey" FOREIGN KEY ("customer_id") REFERENCES "customer" ("customer_id") ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT "payment_rental_id_fkey" FOREIGN KEY ("rental_id") REFERENCES "rental" ("rental_id") ON DELETE
    SET NULL ON UPDATE CASCADE,
        CONSTRAINT "payment_staff_id_fkey" FOREIGN KEY ("staff_id") REFERENCES "staff" ("staff_id") ON DELETE RESTRICT ON UPDATE CASCADE
);