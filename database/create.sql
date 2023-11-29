-- MySQL Script generated by MySQL Workbench
-- ter 28 nov 2023 22:14:51
-- Model: New Model    Version: 1.0
-- MySQL Workbench Forward Engineering

SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0;
SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0;
SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='ONLY_FULL_GROUP_BY,STRICT_TRANS_TABLES,NO_ZERO_IN_DATE,NO_ZERO_DATE,ERROR_FOR_DIVISION_BY_ZERO,NO_ENGINE_SUBSTITUTION';

-- -----------------------------------------------------
-- Schema pet_clinic
-- -----------------------------------------------------

-- -----------------------------------------------------
-- Schema pet_clinic
-- -----------------------------------------------------
CREATE SCHEMA IF NOT EXISTS `pet_clinic` DEFAULT CHARACTER SET utf8 ;
USE `pet_clinic` ;

-- -----------------------------------------------------
-- Table `pet_clinic`.`PetOwner`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `pet_clinic`.`PetOwner` ;

CREATE TABLE IF NOT EXISTS `pet_clinic`.`PetOwner` (
  `id` INT NOT NULL,
  `name` VARCHAR(120) NOT NULL,
  `birth_date` DATE NULL,
  `email` VARCHAR(80) NOT NULL,
  `phone` VARCHAR(45) NULL,
  `address` VARCHAR(120) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE INDEX `id_UNIQUE` (`id` ASC) VISIBLE)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `pet_clinic`.`Pet`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `pet_clinic`.`Pet` ;

CREATE TABLE IF NOT EXISTS `pet_clinic`.`Pet` (
  `id` INT NOT NULL,
  `name` VARCHAR(120) NOT NULL,
  `breed` VARCHAR(45) NULL,
  `age` INT NULL,
  `owner` INT NULL,
  `flag_removed` TINYINT(1) NULL,
  PRIMARY KEY (`id`),
  UNIQUE INDEX `id_UNIQUE` (`id` ASC) VISIBLE)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `pet_clinic`.`Veterinarian`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `pet_clinic`.`Veterinarian` ;

CREATE TABLE IF NOT EXISTS `pet_clinic`.`Veterinarian` (
  `id` INT NOT NULL,
  `name` VARCHAR(120) NOT NULL,
  `inscricaoCRMV` VARCHAR(75) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE INDEX `id_UNIQUE` (`id` ASC) VISIBLE)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `pet_clinic`.`Vaccination`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `pet_clinic`.`Vaccination` ;

CREATE TABLE IF NOT EXISTS `pet_clinic`.`Vaccination` (
  `id` INT NOT NULL,
  `description` VARCHAR(120) NOT NULL,
  `pet` INT NOT NULL,
  UNIQUE INDEX `id_UNIQUE` (`id` ASC) VISIBLE,
  PRIMARY KEY (`id`),
  INDEX `fk_vaccination_pet_idx` (`pet` ASC) VISIBLE,
  CONSTRAINT `fk_vaccination_pet`
    FOREIGN KEY (`pet`)
    REFERENCES `pet_clinic`.`Pet` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `pet_clinic`.`Treatment`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `pet_clinic`.`Treatment` ;

CREATE TABLE IF NOT EXISTS `pet_clinic`.`Treatment` (
  `id` INT NOT NULL,
  `description` VARCHAR(120) NOT NULL,
  `pet` INT NOT NULL,
  `veterinarian` INT NOT NULL,
  UNIQUE INDEX `id_UNIQUE` (`id` ASC) VISIBLE,
  PRIMARY KEY (`id`),
  INDEX `fk_treatment_pet_idx` (`pet` ASC) VISIBLE,
  INDEX `fk_treatement_veterinarian_idx` (`veterinarian` ASC) VISIBLE,
  CONSTRAINT `fk_treatment_pet`
    FOREIGN KEY (`pet`)
    REFERENCES `pet_clinic`.`Pet` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `fk_treatement_veterinarian`
    FOREIGN KEY (`veterinarian`)
    REFERENCES `pet_clinic`.`Veterinarian` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;


SET SQL_MODE=@OLD_SQL_MODE;
SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS;
SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS;
