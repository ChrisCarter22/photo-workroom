# IPTC_FIELD_MAP.md

This document is the canonical field-by-field map for IPTC Photo Metadata support in Photo Workroom.

Current repository status on March 8, 2026:

- no metadata engine exists yet
- this file defines the complete IPTC 2025.1 property inventory the future metadata subsystem must account for
- the canonical standard source is the official IPTC Photo Metadata Standard 2025.1 technical reference JSON
- the local fixture `examples/IPTC_Fields.XMP` is now a parseable XMP sidecar reference for a large subset of the IPTC field surface

## Source of truth

- official IPTC JSON: `https://iptc.org/std/photometadata/specification/iptc-pmd-techreference_2025.1.json`
- IPTC release timestamp: `2025-11-04T09:58:40+00:00`
- IPTC release note: This TechReference release is the initial release of the standard version 2025.1, as of 2025-11-03.
- official inventory counts: `66` top-level properties, `19` structure definitions, and `101` defined structure subfields plus the `AltLang` helper notes

## Mapping conventions

- `Normalized target` is the planned logical field name in the Photo Workroom metadata engine. It is not yet a DB column contract.
- `Direct` means the field should be available in the early first-class metadata UI.
- `Advanced` means the field needs an explicit model and round-trip-safe support, but it may live in advanced or secondary metadata surfaces first.
- `Preserve` means the field must survive read-modify-write cycles even if we do not expose it for editing in the first UI.
- `struct(<name>)` means the top-level property uses one of the structured definitions listed later in this file.
- Alt-language values are modeled as language-keyed text maps such as `metadata.title_alt["x-default"]`.

## Local fixture coverage

- `examples/IPTC_Fields.XMP` currently covers `49` of `66` official top-level IPTC properties.
- It is a strong sidecar fixture for common authoring, rights, location, artwork, registry, release, and party structures.
- It does not yet cover every IPTC property. The missing top-level properties are listed below and should continue to be covered by the official JSON map plus the JPEG reference fixture.

- `aIPromptInformation`: `Iptc4xmpExt:AIPromptInformation`
- `aIPromptWriterNam`: `Iptc4xmpExt:AIPromptWriterName`
- `aISystemUsed`: `Iptc4xmpExt:AISystemUsed`
- `aISystemVersionUsed`: `Iptc4xmpExt:AISystemVersionUsed`
- `aboutCvTerms`: `Iptc4xmpExt:AboutCvTerm`
- `contributors`: `Iptc4xmpExt:Contributor`
- `dataMining`: `plus:DataMining`
- `digitalImageGuid`: `Iptc4xmpExt:DigImageGUID`
- `embdEncRightsExprs`: `Iptc4xmpExt:EmbdEncRightsExpr`
- `eventId`: `Iptc4xmpExt:EventId`
- `genres`: `Iptc4xmpExt:Genre`
- `imageRating`: `xmp:Rating`
- `imageRegion`: `Iptc4xmpExt:ImageRegion`
- `linkedEncRightsExprs`: `Iptc4xmpExt:LinkedEncRightsExpr`
- `otherConstraints`: `plus:OtherConstraints`
- `personsShown`: `Iptc4xmpExt:PersonInImageWDetails`
- `productsShown`: `Iptc4xmpExt:ProductInImage`

## Top-level property map

| Sort | IPTC key | Label | Schema | Type | Occurs | XMP ID | Normalized target | Support |
|---|---|---|---|---|---|---|---|---|
| tlp100 | `additionalModelInfo` | Additional model info | IptcExt | string | single | `Iptc4xmpExt:AddlModelInfo` | `metadata.additional_model_info` | Advanced |
| tlp110 | `aIPromptInformation` | AI Prompt Information | IptcExt | string | single | `Iptc4xmpExt:AIPromptInformation` | `metadata.ai.prompt_information` | Advanced |
| tlp120 | `aIPromptWriterNam` | AI Prompt Writer Name | IptcExt | string | single | `Iptc4xmpExt:AIPromptWriterName` | `metadata.ai.prompt_writer_name` | Advanced |
| tlp130 | `aISystemUsed` | AI System Used | IptcExt | string | single | `Iptc4xmpExt:AISystemUsed` | `metadata.ai.system_used` | Advanced |
| tlp140 | `aISystemVersionUsed` | AI System Version Used | IptcExt | string | single | `Iptc4xmpExt:AISystemVersionUsed` | `metadata.ai.system_version_used` | Advanced |
| tlp150 | `altTextAccessibility` | Alt Text (Accessibility) | IptcCore | struct(AltLang) | single | `Iptc4xmpCore:AltTextAccessibility` | `metadata.accessibility.alt_text_alt` | Direct |
| tlp160 | `artworkOrObjects` | Artwork or object in the image | IptcExt | struct(ArtworkOrObject) | multi | `Iptc4xmpExt:ArtworkOrObject` | `metadata.artwork[]` | Advanced |
| tlp170 | `cityName` | City | IptcCore | string | single | `photoshop:City` | `metadata.location_legacy.city` | Direct |
| tlp180 | `organisationInImageCodes` | Code of featured Organisation | IptcExt | string | multi | `Iptc4xmpExt:OrganisationInImageCode` | `metadata.depicted_organisations.codes[]` | Advanced |
| tlp190 | `contributors` | Contributor | IptcExt | struct(EntityWRole) | multi | `Iptc4xmpExt:Contributor` | `metadata.contributors[]` | Advanced |
| tlp200 | `copyrightNotice` | Copyright Notice | IptcCore | struct(AltLang) | single | `dc:rights` | `metadata.copyright_notice_alt` | Direct |
| tlp210 | `copyrightOwners` | Copyright owner | IptcExt | struct(CopyrightOwner) | multi | `plus:CopyrightOwner` | `metadata.rights_parties.copyright_owners[]` | Advanced |
| tlp220 | `countryName` | Country | IptcCore | string | single | `photoshop:Country` | `metadata.location_legacy.country` | Direct |
| tlp230 | `countryCode` | ISO Country Code | IptcCore | string | single | `Iptc4xmpCore:CountryCode` | `metadata.location_legacy.country_code` | Direct |
| tlp240 | `creatorNames` | Creator | IptcCore | string | multi | `dc:creator` | `metadata.creator_names[]` | Direct |
| tlp250 | `creatorContactInfo` | Creator's Contact info | IptcCore | struct(CreatorContactInfo) | single | `Iptc4xmpCore:CreatorContactInfo` | `metadata.creator_contact` | Advanced |
| tlp260 | `jobtitle` | Creator's Jobtitle | IptcCore | string | single | `photoshop:AuthorsPosition` | `metadata.creator_job_title` | Direct |
| tlp270 | `creditLine` | Credit Line | IptcCore | string | single | `photoshop:Credit` | `metadata.credit_line` | Direct |
| tlp280 | `aboutCvTerms` | CV-Term About Image | IptcExt | struct(CvTerm) | multi | `Iptc4xmpExt:AboutCvTerm` | `metadata.about_cv_terms[]` | Advanced |
| tlp290 | `dataMining` | Data Mining | IptcExt | string | single | `plus:DataMining` | `metadata.data_mining` | Advanced |
| tlp300 | `dateCreated` | Date Created | IptcCore | string | single | `photoshop:DateCreated` | `metadata.date_created` | Direct |
| tlp310 | `description` | Caption/Description | IptcCore | struct(AltLang) | single | `dc:description` | `metadata.description_alt` | Direct |
| tlp320 | `captionWriter` | Caption/Description writer | IptcCore | string | single | `photoshop:CaptionWriter` | `metadata.description_writer` | Direct |
| tlp330 | `digitalImageGuid` | Digital Image Identifier | IptcExt | string | single | `Iptc4xmpExt:DigImageGUID` | `metadata.digital_image_guid` | Advanced |
| tlp340 | `digitalSourceType` | Type of source for this photo | IptcExt | string | single | `Iptc4xmpExt:DigitalSourceType` | `metadata.digital_source_type` | Direct |
| tlp350 | `embdEncRightsExprs` | Embedded Encoded Rights Expression | IptcExt | struct(EmbdEncRightsExpr) | multi | `Iptc4xmpExt:EmbdEncRightsExpr` | `metadata.rights_expressions.embedded[]` | Advanced |
| tlp360 | `eventId` | Event ID | IptcExt | string | multi | `Iptc4xmpExt:EventId` | `metadata.event_ids[]` | Advanced |
| tlp370 | `eventName` | Event Name | IptcExt | struct(AltLang) | single | `Iptc4xmpExt:Event` | `metadata.event_name_alt` | Direct |
| tlp380 | `extDescrAccessibility` | Extended Description (Accessibility) | IptcCore | struct(AltLang) | single | `Iptc4xmpCore:ExtDescrAccessibility` | `metadata.accessibility.extended_description_alt` | Direct |
| tlp390 | `genres` | Genre | IptcExt | struct(CvTerm) | multi | `Iptc4xmpExt:Genre` | `metadata.genres[]` | Advanced |
| tlp400 | `headline` | Headline | IptcCore | string | single | `photoshop:Headline` | `metadata.headline` | Direct |
| tlp410 | `imageCreators` | Image Creator | IptcExt | struct(ImageCreator) | multi | `plus:ImageCreator` | `metadata.rights_parties.image_creators[]` | Advanced |
| tlp420 | `imageRating` | Rating | IptcExt | number | single | `xmp:Rating` | `metadata.rating` | Direct |
| tlp430 | `imageRegion` | Image Region(s) | IptcExt | struct(ImageRegion) | multi | `Iptc4xmpExt:ImageRegion` | `metadata.regions[]` | Advanced |
| tlp440 | `registryEntries` | Registry Entry | IptcExt | struct(RegistryEntry) | multi | `Iptc4xmpExt:RegistryId` | `metadata.registry_entries[]` | Advanced |
| tlp450 | `suppliers` | Image Supplier | IptcExt | struct(ImageSupplier) | multi | `plus:ImageSupplier` | `metadata.rights_parties.image_suppliers[]` | Advanced |
| tlp460 | `imageSupplierImageId` | Image Supplier Image Id | IptcExt | string | single | `plus:ImageSupplierImageID` | `metadata.image_supplier_image_id` | Advanced |
| tlp470 | `instructions` | Instructions | IptcCore | string | single | `photoshop:Instructions` | `metadata.instructions` | Direct |
| tlp480 | `intellectualGenre` | Intellectual genre | IptcCore | string | single | `Iptc4xmpCore:IntellectualGenre` | `metadata.intellectual_genre` | Advanced |
| tlp490 | `jobid` | Job Identifier | IptcCore | string | single | `photoshop:TransmissionReference` | `metadata.job_id` | Direct |
| tlp500 | `keywords` | Keywords | IptcCore | string | multi | `dc:subject` | `metadata.keywords[]` | Direct |
| tlp510 | `licensors` | Licensor | IptcExt | struct(Licensor) | multi | `plus:Licensor` | `metadata.rights_parties.licensors[]` | Advanced |
| tlp520 | `linkedEncRightsExprs` | Linked Encoded Rights Expression | IptcExt | struct(LinkedEncRightsExpr) | multi | `Iptc4xmpExt:LinkedEncRightsExpr` | `metadata.rights_expressions.linked[]` | Advanced |
| tlp530 | `locationCreated` | Location Created | IptcExt | struct(Location) | multi | `Iptc4xmpExt:LocationCreated` | `metadata.locations_created[]` | Advanced |
| tlp540 | `locationsShown` | Location shown | IptcExt | struct(Location) | multi | `Iptc4xmpExt:LocationShown` | `metadata.locations_shown[]` | Advanced |
| tlp550 | `maxAvailHeight` | Maximum available height | IptcExt | number | single | `Iptc4xmpExt:MaxAvailHeight` | `metadata.max_available.height` | Preserve |
| tlp560 | `maxAvailWidth` | Maximum available width | IptcExt | number | single | `Iptc4xmpExt:MaxAvailWidth` | `metadata.max_available.width` | Preserve |
| tlp570 | `minorModelAgeDisclosure` | Minor Model Age Disclosure | IptcExt | string | single | `plus:MinorModelAgeDisclosure` | `metadata.minor_model_age_disclosure` | Advanced |
| tlp580 | `modelAges` | Model age | IptcExt | number | multi | `Iptc4xmpExt:ModelAge` | `metadata.model_ages[]` | Advanced |
| tlp590 | `modelReleaseDocuments` | Model Release Id | IptcExt | string | multi | `plus:ModelReleaseID` | `metadata.model_release_ids[]` | Advanced |
| tlp600 | `modelReleaseStatus` | Model Release Status | IptcExt | string | single | `plus:ModelReleaseStatus` | `metadata.model_release_status` | Advanced |
| tlp610 | `organisationInImageNames` | Name of featured Organisation | IptcExt | string | multi | `Iptc4xmpExt:OrganisationInImageName` | `metadata.depicted_organisations.names[]` | Advanced |
| tlp620 | `otherConstraints` | Constraint | IptcExt | struct(AltLang) | single | `plus:OtherConstraints` | `metadata.other_constraints_alt` | Advanced |
| tlp630 | `personInImageNames` | Person shown | IptcExt | string | multi | `Iptc4xmpExt:PersonInImage` | `metadata.persons_in_image_names[]` | Advanced |
| tlp640 | `personsShown` | Person Shown (Details) | IptcExt | struct(PersonWDetails) | multi | `Iptc4xmpExt:PersonInImageWDetails` | `metadata.persons_shown[]` | Advanced |
| tlp660 | `productsShown` | Product Shown | IptcExt | struct(ProductWGtin) | multi | `Iptc4xmpExt:ProductInImage` | `metadata.products_shown[]` | Advanced |
| tlp670 | `propertyReleaseDocuments` | Property Release Id | IptcExt | string | multi | `plus:PropertyReleaseID` | `metadata.property_release_ids[]` | Advanced |
| tlp680 | `propertyReleaseStatus` | Property Release Status | IptcExt | string | single | `plus:PropertyReleaseStatus` | `metadata.property_release_status` | Advanced |
| tlp690 | `provinceState` | Province/State | IptcCore | string | single | `photoshop:State` | `metadata.location_legacy.province_state` | Direct |
| tlp700 | `usageTerms` | Rights Usage Terms | IptcCore | struct(AltLang) | single | `xmpRights:UsageTerms` | `metadata.usage_terms_alt` | Direct |
| tlp710 | `sceneCodes` | IPTC Scene Code | IptcCore | string | multi | `Iptc4xmpCore:Scene` | `metadata.scene_codes[]` | Advanced |
| tlp720 | `source` | Source (Supply Chain) | IptcCore | string | single | `photoshop:Source` | `metadata.source` | Direct |
| tlp730 | `subjectCodes` | IPTC Subject Code | IptcCore | string | multi | `Iptc4xmpCore:SubjectCode` | `metadata.subject_codes[]` | Advanced |
| tlp740 | `sublocationName` | Sublocation | IptcCore | string | single | `Iptc4xmpCore:Location` | `metadata.location_legacy.sublocation` | Direct |
| tlp750 | `title` | Title | IptcCore | struct(AltLang) | single | `dc:title` | `metadata.title_alt` | Direct |
| tlp760 | `webstatementRights` | Copyright Info URL | IptcExt | string | single | `xmpRights:WebStatement` | `metadata.web_statement_rights` | Direct |

## Structure map

### AltLang

- This is the IPTC helper structure for language-keyed text, not a standalone business object.
- Planned root: `<parent alt-lang text map>`

| Subfield | Meaning | Relative target |
|---|---|---|
| `Note` | A special structure covering variants of a text in different languages. | `n/a` |
| `BCP47langid_1` | Text in the human language corresponding to the BCP 47 language id | `<parent>[lang]` |
| `BCP47langid_toMany` | Text in the human language corresponding to the BCP 47 language id | `<parent>[lang]` |

### ArtworkOrObject

- Planned root: `metadata.artwork[]`

| Sort | Subfield | Label | Type | Occurs | XMP ID or tag | Relative target |
|---|---|---|---|---|---|---|
| s0201 | `circaDateCreated` | Circa Date Created | string | single | `Iptc4xmpExt:AOCircaDateCreated` | `circa_date_created` |
| s0202 | `contentDescription` | Content Description | struct(AltLang) | single | `Iptc4xmpExt:AOContentDescription` | `content_description_alt` |
| s0203 | `contributionDescription` | Contribution Description | struct(AltLang) | single | `Iptc4xmpExt:AOContributionDescription` | `contribution_description_alt` |
| s0204 | `copyrightNotice` | (Artwork or Object detail:) Copyright notice | string | single | `Iptc4xmpExt:AOCopyrightNotice` | `copyright_notice` |
| s0205 | `creatorNames` | (Artwork or Object detail:) Creator | string | multi | `Iptc4xmpExt:AOCreator` | `creator_names[]` |
| s0206 | `creatorIdentifiers` | Creator ID | string | multi | `Iptc4xmpExt:AOCreatorId` | `creator_ids[]` |
| s0207 | `currentCopyrightOwnerIdentifier` | Current Copyright Owner ID | string | single | `Iptc4xmpExt:AOCurrentCopyrightOwnerId` | `current_copyright_owner_id` |
| s0208 | `currentCopyrightOwnerName` | Current Copyright Owner Name | string | single | `Iptc4xmpExt:AOCurrentCopyrightOwnerName` | `current_copyright_owner_name` |
| s0209 | `currentLicensorIdentifier` | Current Licensor ID | string | single | `Iptc4xmpExt:AOCurrentLicensorId` | `current_licensor_id` |
| s0210 | `currentLicensorName` | Current Licensor Name | string | single | `Iptc4xmpExt:AOCurrentLicensorName` | `current_licensor_name` |
| s0211 | `dateCreated` | (Artwork or Object detail:) Date created | string | single | `Iptc4xmpExt:AODateCreated` | `date_created` |
| s0212 | `physicalDescription` | Physical Description | struct(AltLang) | single | `Iptc4xmpExt:AOPhysicalDescription` | `physical_description_alt` |
| s0213 | `source` | (Artwork or Object detail:) Source | string | single | `Iptc4xmpExt:AOSource` | `source` |
| s0214 | `sourceInventoryNr` | (Artwork or Object detail:) Source inventory number | string | single | `Iptc4xmpExt:AOSourceInvNo` | `source_inventory_number` |
| s0215 | `sourceInventoryUrl` | Source Inventory URL | string | single | `Iptc4xmpExt:AOSourceInvURL` | `source_inventory_url` |
| s0216 | `stylePeriod` | Style Period | string | multi | `Iptc4xmpExt:AOStylePeriod` | `style_periods[]` |
| s0217 | `title` | (Artwork or Object detail:) Title | struct(AltLang) | single | `Iptc4xmpExt:AOTitle` | `title_alt` |

### CopyrightOwner

- Planned root: `metadata.rights_parties.copyright_owners[]`

| Sort | Subfield | Label | Type | Occurs | XMP ID or tag | Relative target |
|---|---|---|---|---|---|---|
| s3201 | `copyrightOwnerId` | Copyright Owner ID | string | single | `plus:CopyrightOwnerID` | `id` |
| s3202 | `copyrightOwnerName` | Copyright Owner Name | string | single | `plus:CopyrightOwnerName` | `name` |

### CreatorContactInfo

- Planned root: `metadata.creator_contact`

| Sort | Subfield | Label | Type | Occurs | XMP ID or tag | Relative target |
|---|---|---|---|---|---|---|
| s0101 | `address` | (Contact Info detail:) Address | string | single | `Iptc4xmpCore:CiAdrExtadr` | `address` |
| s0102 | `city` | (Contact Info detail:) City | string | single | `Iptc4xmpCore:CiAdrCity` | `city` |
| s0103 | `country` | (Contact Info detail:) Country | string | single | `Iptc4xmpCore:CiAdrCtry` | `country` |
| s0104 | `emailwork` | (Contact Info detail:) Email(s) | string | single | `Iptc4xmpCore:CiEmailWork` | `email_work` |
| s0105 | `phonework` | (Contact Info detail:) Phone(s) | string | single | `Iptc4xmpCore:CiTelWork` | `phone_work` |
| s0106 | `postalCode` | (Contact Info detail:) Postal Code | string | single | `Iptc4xmpCore:CiAdrPcode` | `postal_code` |
| s0107 | `region` | (Contact Info detail:) State/Province | string | single | `Iptc4xmpCore:CiAdrRegion` | `region` |
| s0108 | `weburlwork` | (Contact Info detail:) Web URL(s) | string | single | `Iptc4xmpCore:CiUrlWork` | `web_urls_work` |

### CvTerm

- Planned root: `metadata.cv_term`

| Sort | Subfield | Label | Type | Occurs | XMP ID or tag | Relative target |
|---|---|---|---|---|---|---|
| s0301 | `cvId` | CV ID | string | single | `Iptc4xmpExt:CvId` | `cv_id` |
| s0302 | `cvTermId` | Term ID | string | single | `Iptc4xmpExt:CvTermId` | `term_id` |
| s0303 | `cvTermName` | Name | struct(AltLang) | single | `Iptc4xmpExt:CvTermName` | `name_alt` |
| s0304 | `cvTermRefinedAbout` | Refined Aboutness | string | single | `Iptc4xmpExt:CvTermRefinedAbout` | `refined_about` |

### EmbdEncRightsExpr

- Planned root: `metadata.rights_expressions.embedded[]`

| Sort | Subfield | Label | Type | Occurs | XMP ID or tag | Relative target |
|---|---|---|---|---|---|---|
| s0401 | `encRightsExpr` | Rights expression | string | single | `Iptc4xmpExt:EncRightsExpr` | `encoded_expression` |
| s0402 | `rightsExprEncType` | Encoding type | string | single | `Iptc4xmpExt:RightsExprEncType` | `encoding_type` |
| s0403 | `rightsExprLangId` | Rights expression language ID | string | single | `Iptc4xmpExt:RightsExprLangId` | `language_id` |

### Entity

- Planned root: `metadata.entity`

| Sort | Subfield | Label | Type | Occurs | XMP ID or tag | Relative target |
|---|---|---|---|---|---|---|
| s0401 | `identifiers` | Identifier | string | multi | `xmp:Identifier` | `identifiers[]` |
| s0402 | `name` | Name | struct(AltLang) | single | `Iptc4xmpExt:Name` | `name_alt` |

### EntityWRole

- Planned root: `metadata.entity_with_role`

| Sort | Subfield | Label | Type | Occurs | XMP ID or tag | Relative target |
|---|---|---|---|---|---|---|
| s9201 | `identifiers` | Identifier | string | multi | `xmp:Identifier` | `identifiers[]` |
| s9202 | `name` | Name | struct(AltLang) | single | `Iptc4xmpExt:Name` | `name_alt` |
| s9203 | `role` | Role | string | multi | `Iptc4xmpExt:Role` | `roles[]` |

### ImageCreator

- Planned root: `metadata.rights_parties.image_creators[]`

| Sort | Subfield | Label | Type | Occurs | XMP ID or tag | Relative target |
|---|---|---|---|---|---|---|
| s3301 | `imageCreatorId` | Image Creator ID | string | single | `plus:ImageCreatorID` | `id` |
| s3302 | `imageCreatorName` | Image Creator Name | string | single | `plus:ImageCreatorName` | `name` |

### ImageRegion

- Planned root: `metadata.regions[]`

| Sort | Subfield | Label | Type | Occurs | XMP ID or tag | Relative target |
|---|---|---|---|---|---|---|
| s0501 | `regionBoundary` | Region Boundary | struct(RegionBoundary) | single | `Iptc4xmpExt:RegionBoundary` | `boundary` |
| s0502 | `rId` | Identifier | string | single | `Iptc4xmpExt:rId` | `id` |
| s0503 | `name` | Name | struct(AltLang) | single | `Iptc4xmpExt:Name` | `name_alt` |
| s0504 | `rCtype` | Content Type | struct(Entity) | multi | `Iptc4xmpExt:rCtype` | `content_types[]` |
| s0505 | `rRole` | Role | struct(Entity) | multi | `Iptc4xmpExt:rRole` | `roles[]` |
| s0506 | `$anypmdproperty` | {as defined for the used metadata property} | any | single |  | `other_metadata` |

### ImageSupplier

- Planned root: `metadata.rights_parties.image_suppliers[]`

| Sort | Subfield | Label | Type | Occurs | XMP ID or tag | Relative target |
|---|---|---|---|---|---|---|
| s3401 | `imageSupplierId` | Image Supplier ID | string | single | `plus:ImageSupplierID` | `id` |
| s3402 | `imageSupplierName` | Image Supplier Name | string | single | `plus:ImageSupplierName` | `name` |

### Licensor

- Planned root: `metadata.rights_parties.licensors[]`

| Sort | Subfield | Label | Type | Occurs | XMP ID or tag | Relative target |
|---|---|---|---|---|---|---|
| s3101 | `licensorId` | Licensor ID | string | single | `plus:LicensorID` | `id` |
| s3102 | `licensorName` | Licensor Name | string | single | `plus:LicensorName` | `name` |
| s3103 | `licensorAddress` | Licensor Address | string | single | `plus:LicensorStreetAddress` | `street_address` |
| s3104 | `licensorAddressDetail` | Licensor Adress Detail | string | single | `plus:LicensorExtendedAddress` | `extended_address` |
| s3105 | `licensorCity` | Licensor City | string | single | `plus:LicensorCity` | `city` |
| s3106 | `licensorStateProvince` | Licensor State or Province | string | single | `plus:LicensorRegion` | `region` |
| s3107 | `licensorPostalCode` | Licensor Postal Code | string | single | `plus:LicensorPostalCode` | `postal_code` |
| s3108 | `licensorCountryName` | Licensor Country | string | single | `plus:LicensorCountry` | `country` |
| s3109 | `licensorTelephoneType1` | Licensor Telephone Type 1 | string | single | `plus:LicensorTelephoneType1` | `telephone_type_1` |
| s3110 | `licensorTelephone1` | Licensor Telephone 1 | string | single | `plus:LicensorTelephone1` | `telephone_1` |
| s3111 | `licensorTelephoneType2` | Licensor Telephone Type 2 | string | single | `plus:LicensorTelephoneType2` | `telephone_type_2` |
| s3112 | `licensorTelephone2` | Licensor Telephone 2 | string | single | `plus:LicensorTelephone2` | `telephone_2` |
| s3113 | `licensorEmail` | Licensor Email | string | single | `plus:LicensorEmail` | `email` |
| s3114 | `licensorUrl` | Licensor URL | string | single | `plus:LicensorURL` | `url` |

### LinkedEncRightsExpr

- Planned root: `metadata.rights_expressions.linked[]`

| Sort | Subfield | Label | Type | Occurs | XMP ID or tag | Relative target |
|---|---|---|---|---|---|---|
| s0801 | `linkedRightsExpr` | Link to Rights Expression | string | single | `Iptc4xmpExt:LinkedRightsExpr` | `link` |
| s0802 | `rightsExprEncType` | Encoding type | string | single | `Iptc4xmpExt:RightsExprEncType` | `encoding_type` |
| s0803 | `rightsExprLangId` | Rights Expression Language ID | string | single | `Iptc4xmpExt:RightsExprLangId` | `language_id` |

### Location

- Planned root: `metadata.locations[]`

| Sort | Subfield | Label | Type | Occurs | XMP ID or tag | Relative target |
|---|---|---|---|---|---|---|
| s0901 | `identifiers` | Location ID | string | multi | `Iptc4xmpExt:LocationId` | `identifiers[]` |
| s0902 | `name` | (Location detail:) Location Name | struct(AltLang) | single | `Iptc4xmpExt:LocationName` | `name_alt` |
| s0903 | `sublocation` | (Location detail:) Sublocation | string | single | `Iptc4xmpExt:Sublocation` | `sublocation` |
| s0904 | `city` | (Location detail:) City | string | single | `Iptc4xmpExt:City` | `city` |
| s0905 | `provinceState` | (Location detail:) Province/State | string | single | `Iptc4xmpExt:ProvinceState` | `province_state` |
| s0906 | `countryName` | (Location detail:) Country Name | string | single | `Iptc4xmpExt:CountryName` | `country_name` |
| s0907 | `countryCode` | (Location detail:) Country ISO-Code | string | single | `Iptc4xmpExt:CountryCode` | `country_code` |
| s0908 | `worldRegion` | (Location detail:) World Region | string | single | `Iptc4xmpExt:WorldRegion` | `world_region` |
| s0911 | `gpsAltitude` | (Location detail:) GPS-Altitude | number | single | `exif:GPSAltitude` | `gps.altitude` |
| s0912 | `gpsAltitudeRef` | (Location detail:) GPS-Altitude Ref | number | single | `exif:GPSAltitudeRef` | `gps.altitude_ref` |
| s0913 | `gpsLongitude` | (Location detail:) GPS-Longitude | number | single | `exif:GPSLongitude` | `gps.longitude` |
| s0914 | `gpsLatitude` | (Location detail:) GPS-Lattitude | number | single | `exif:GPSLatitude` | `gps.latitude` |

### PersonWDetails

- Planned root: `metadata.persons_shown[]`

| Sort | Subfield | Label | Type | Occurs | XMP ID or tag | Relative target |
|---|---|---|---|---|---|---|
| s1001 | `identifiers` | Identifier | string | multi | `Iptc4xmpExt:PersonId` | `identifiers[]` |
| s1002 | `name` | Name | struct(AltLang) | single | `Iptc4xmpExt:PersonName` | `name_alt` |
| s1003 | `description` | Description | struct(AltLang) | single | `Iptc4xmpExt:PersonDescription` | `description_alt` |
| s1004 | `characteristics` | Characteristics | struct(CvTerm) | multi | `Iptc4xmpExt:PersonCharacteristic` | `characteristics[]` |

### ProductWGtin

- Planned root: `metadata.products_shown[]`

| Sort | Subfield | Label | Type | Occurs | XMP ID or tag | Relative target |
|---|---|---|---|---|---|---|
| s1101 | `gtin` | GTIN | string | single | `Iptc4xmpExt:ProductGTIN` | `gtin` |
| s1102 | `identifiers` | Identifier | string | multi | `Iptc4xmpExt:ProductId` | `identifiers[]` |
| s1103 | `name` | Name | struct(AltLang) | single | `Iptc4xmpExt:ProductName` | `name_alt` |
| s1104 | `description` | Description | struct(AltLang) | single | `Iptc4xmpExt:ProductDescription` | `description_alt` |

### RegionBoundary

- Planned root: `metadata.regions[].boundary`

| Sort | Subfield | Label | Type | Occurs | XMP ID or tag | Relative target |
|---|---|---|---|---|---|---|
| s0601 | `rbShape` | Shape | string | single | `Iptc4xmpExt:rbShape` | `shape` |
| s0602 | `rbUnit` | Measuring Unit | string | single | `Iptc4xmpExt:rbUnit` | `unit` |
| s0603 | `rbX` | X-Axis Coordinate | number | single | `Iptc4xmpExt:rbX` | `x` |
| s0604 | `rbY` | Y-Axis Coordinate | number | single | `Iptc4xmpExt:rbY` | `y` |
| s0605 | `rbW` | Rectangle Width | number | single | `Iptc4xmpExt:rbW` | `width` |
| s0606 | `rbH` | Rectangle Height | number | single | `Iptc4xmpExt:rbH` | `height` |
| s0607 | `rbRx` | Circle Radius | number | single | `Iptc4xmpExt:rbRx` | `radius` |
| s0608 | `rbVertices` | Polygon Vertices | struct(RegionBoundaryPoint) | multi | `Iptc4xmpExt:rbVertices` | `vertices[]` |

### RegionBoundaryPoint

- Planned root: `metadata.regions[].boundary.vertices[]`

| Sort | Subfield | Label | Type | Occurs | XMP ID or tag | Relative target |
|---|---|---|---|---|---|---|
| s0701 | `rbX` | X-Axis Coordinate | number | single | `Iptc4xmpExt:rbX` | `x` |
| s0702 | `rbY` | Y-Axis Coordinate | number | single | `Iptc4xmpExt:rbY` | `y` |

### RegistryEntry

- Planned root: `metadata.registry_entries[]`

| Sort | Subfield | Label | Type | Occurs | XMP ID or tag | Relative target |
|---|---|---|---|---|---|---|
| s1201 | `registryIdentifier` | (Registry Entry detail:) Organisation Identifier | string | single | `Iptc4xmpExt:RegOrgId` | `registry_identifier` |
| s1202 | `assetIdentifier` | (Registry Entry detail:) Item Identifier | string | single | `Iptc4xmpExt:RegItemId` | `asset_identifier` |
| s1203 | `role` | (Registry Entry detail:) Role | string | single | `Iptc4xmpExt:RegEntryRole` | `role` |

## Implementation rules

- Every top-level IPTC property above must be readable and name-addressable in the metadata engine, even if the first UI does not expose it.
- Every structured IPTC property above must round-trip without loss when the underlying metadata library supports non-destructive writes.
- Unsupported IPTC fields must never be dropped silently during write-back.
- The XMP fixture `examples/IPTC_Fields.XMP` should be expanded over time until it covers the remaining missing top-level properties or those gaps are covered by additional explicit fixtures.
