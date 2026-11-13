# Changelog

## [0.31.4](https://github.com/meteroid-oss/meteroid-rust/compare/v0.31.3...v0.31.4) (2026-11-13)


### Features

* **api:** update `GET /api/v1/licenses`, `GET /api/v1/licenses/{license_id}`, `POST /api/v1/licenses`, `POST /api/v1/licenses/{license_id}/revoke` ([3de55b4](https://github.com/meteroid-oss/meteroid-rust/commit/3de55b438f9cb84b3ad4ad4a19a4417f5c97f234))
* **api:** update SDKs to meteroid 0.1.0 ([#24](https://github.com/meteroid-oss/meteroid-rust/issues/24)) ([3de55b4](https://github.com/meteroid-oss/meteroid-rust/commit/3de55b438f9cb84b3ad4ad4a19a4417f5c97f234))

## [0.31.3](https://github.com/meteroid-oss/meteroid-rust/compare/v0.31.2...v0.31.3) (2026-11-06)


### Features

* **api:** update `GET /api/v1/usage/customer/{customer_id}/costs`, `GET /api/v1/usage/customer/{customer_id}/events`, `GET /api/v1/usage/subscription/{subscription_id}/costs`, `GET /api/v1/usage/subscription/{subscription_id}/events`, `POST webhook:payment.failed` and 5 more ([0ba8273](https://github.com/meteroid-oss/meteroid-rust/commit/0ba82731d0b9a2dc55b3f8071b132c897629ff2a))
* **api:** update SDKs to meteroid 0.1.0 ([#22](https://github.com/meteroid-oss/meteroid-rust/issues/22)) ([0ba8273](https://github.com/meteroid-oss/meteroid-rust/commit/0ba82731d0b9a2dc55b3f8071b132c897629ff2a))

## [0.31.2](https://github.com/meteroid-oss/meteroid-rust/compare/v0.31.1...v0.31.2) (2026-11-05)


### Features

* **api:** update `PATCH /api/v1/customers/{id_or_alias}/budgets/{budget_id}`, `PATCH /api/v1/customers/{id_or_alias}/wallets/{currency}`, `POST /api/v1/customers/{id_or_alias}/budgets`, `PUT /api/v1/plans/versions/{plan_version_id}/minimum`, `PUT /api/v1/plans/{plan_id}` ([709f687](https://github.com/meteroid-oss/meteroid-rust/commit/709f68780964629014b67ed1fe09f76348f64727))
* **api:** update SDKs to meteroid 0.1.0 ([#20](https://github.com/meteroid-oss/meteroid-rust/issues/20)) ([709f687](https://github.com/meteroid-oss/meteroid-rust/commit/709f68780964629014b67ed1fe09f76348f64727))

## [0.31.1](https://github.com/meteroid-oss/meteroid-rust/compare/v0.31.0...v0.31.1) (2026-10-30)


### Features

* **api:** add `GET /api/v1/licenses`, `GET /api/v1/licenses/{license_id}`, `GET /api/v1/licenses/{license_id}/activation-code`, `GET /api/v1/licensing/signing-keys`, `POST /api/v1/licenses` and 2 more ([7b390a7](https://github.com/meteroid-oss/meteroid-rust/commit/7b390a7ade0826d453c58e206336ef482599d451))
* **api:** update `GET /api/v1/plans`, `GET /api/v1/plans/{plan_id}`, `PATCH /api/v1/plans/{plan_id}`, `POST /api/v1/custom-property-definitions`, `POST /api/v1/plans` and 3 more ([7b390a7](https://github.com/meteroid-oss/meteroid-rust/commit/7b390a7ade0826d453c58e206336ef482599d451))
* **api:** update SDKs to meteroid 0.1.0 ([#18](https://github.com/meteroid-oss/meteroid-rust/issues/18)) ([7b390a7](https://github.com/meteroid-oss/meteroid-rust/commit/7b390a7ade0826d453c58e206336ef482599d451))

## [0.31.0](https://github.com/meteroid-oss/meteroid-rust/compare/v0.30.1...v0.31.0) (2026-10-28)


### ⚠ BREAKING CHANGES

* **api:** update SDKs to meteroid 0.1.0 ([#16](https://github.com/meteroid-oss/meteroid-rust/issues/16))
* **api:** `GET /api/v1/coupons`: added `#/components/schemas/CreditDiscount` to the `data/items/discount` response property `oneOf` list for the response status `200`
* **api:** `GET /api/v1/coupons/{coupon_id}`: added `#/components/schemas/CreditDiscount` to the `discount` response property `oneOf` list for the response status `200`
* **api:** `GET /api/v1/plans`: added `#/components/schemas/CreditPlanFee` to the `data/items/price_components/items/fee/oneOf[#/components/schemas/Fee]/` response property `oneOf` list for the response status `200`
* **api:** `GET /api/v1/plans/{plan_id}`: added `#/components/schemas/CreditPlanFee` to the `price_components/items/fee/oneOf[#/components/schemas/Fee]/` response property `oneOf` list for the response status `200`
* **api:** `GET /api/v1/products`: added `#/components/schemas/CreditFeeStructure` to the `data/items/fee_structure` response property `oneOf` list for the response status `200`
* **api:** `GET /api/v1/products/{product_id}`: added `#/components/schemas/CreditFeeStructure` to the `fee_structure` response property `oneOf` list for the response status `200`
* **api:** `GET /api/v1/subscriptions/{subscription_id}`: added `#/components/schemas/CreditDiscount` to the `applied_coupons/items/coupon/discount` response property `oneOf` list for the response status `200`
* **api:** `GET /api/v1/subscriptions/{subscription_id}`: added `#/components/schemas/CreditFee` to the `add_ons/items/fee` response property `oneOf` list for the response status `200`
* **api:** `GET /api/v1/subscriptions/{subscription_id}`: added `#/components/schemas/CreditFee` to the `components/items/fee` response property `oneOf` list for the response status `200`
* **api:** `PATCH /api/v1/coupons/{coupon_id}`: added the new required request property `discount/oneOf[#/components/schemas/CouponDiscount]/oneOf[#/components/schemas/FixedDiscount]/type`
* **api:** 90 more breaking changes

### Features

* **api:** `GET /api/v1/coupons/{coupon_id}`: added `#/components/schemas/CreditDiscount` to the `discount` response property `oneOf` list for the response status `200` ([f385efa](https://github.com/meteroid-oss/meteroid-rust/commit/f385efa4270f0a72b6a55ce3aaef7e982e1a12bb))
* **api:** `GET /api/v1/coupons`: added `#/components/schemas/CreditDiscount` to the `data/items/discount` response property `oneOf` list for the response status `200` ([f385efa](https://github.com/meteroid-oss/meteroid-rust/commit/f385efa4270f0a72b6a55ce3aaef7e982e1a12bb))
* **api:** `GET /api/v1/plans/{plan_id}`: added `#/components/schemas/CreditPlanFee` to the `price_components/items/fee/oneOf[#/components/schemas/Fee]/` response property `oneOf` list for the response status `200` ([f385efa](https://github.com/meteroid-oss/meteroid-rust/commit/f385efa4270f0a72b6a55ce3aaef7e982e1a12bb))
* **api:** `GET /api/v1/plans`: added `#/components/schemas/CreditPlanFee` to the `data/items/price_components/items/fee/oneOf[#/components/schemas/Fee]/` response property `oneOf` list for the response status `200` ([f385efa](https://github.com/meteroid-oss/meteroid-rust/commit/f385efa4270f0a72b6a55ce3aaef7e982e1a12bb))
* **api:** `GET /api/v1/products/{product_id}`: added `#/components/schemas/CreditFeeStructure` to the `fee_structure` response property `oneOf` list for the response status `200` ([f385efa](https://github.com/meteroid-oss/meteroid-rust/commit/f385efa4270f0a72b6a55ce3aaef7e982e1a12bb))
* **api:** `GET /api/v1/products`: added `#/components/schemas/CreditFeeStructure` to the `data/items/fee_structure` response property `oneOf` list for the response status `200` ([f385efa](https://github.com/meteroid-oss/meteroid-rust/commit/f385efa4270f0a72b6a55ce3aaef7e982e1a12bb))
* **api:** `GET /api/v1/subscriptions/{subscription_id}`: added `#/components/schemas/CreditDiscount` to the `applied_coupons/items/coupon/discount` response property `oneOf` list for the response status `200` ([f385efa](https://github.com/meteroid-oss/meteroid-rust/commit/f385efa4270f0a72b6a55ce3aaef7e982e1a12bb))
* **api:** `GET /api/v1/subscriptions/{subscription_id}`: added `#/components/schemas/CreditFee` to the `add_ons/items/fee` response property `oneOf` list for the response status `200` ([f385efa](https://github.com/meteroid-oss/meteroid-rust/commit/f385efa4270f0a72b6a55ce3aaef7e982e1a12bb))
* **api:** `GET /api/v1/subscriptions/{subscription_id}`: added `#/components/schemas/CreditFee` to the `components/items/fee` response property `oneOf` list for the response status `200` ([f385efa](https://github.com/meteroid-oss/meteroid-rust/commit/f385efa4270f0a72b6a55ce3aaef7e982e1a12bb))
* **api:** `PATCH /api/v1/coupons/{coupon_id}`: added the new required request property `discount/oneOf[#/components/schemas/CouponDiscount]/oneOf[#/components/schemas/FixedDiscount]/type` ([f385efa](https://github.com/meteroid-oss/meteroid-rust/commit/f385efa4270f0a72b6a55ce3aaef7e982e1a12bb))
* **api:** 90 more breaking changes ([f385efa](https://github.com/meteroid-oss/meteroid-rust/commit/f385efa4270f0a72b6a55ce3aaef7e982e1a12bb))
* **api:** update SDKs to meteroid 0.1.0 ([#16](https://github.com/meteroid-oss/meteroid-rust/issues/16)) ([f385efa](https://github.com/meteroid-oss/meteroid-rust/commit/f385efa4270f0a72b6a55ce3aaef7e982e1a12bb))
* **api:** update the API definitions ([f385efa](https://github.com/meteroid-oss/meteroid-rust/commit/f385efa4270f0a72b6a55ce3aaef7e982e1a12bb))

## [0.30.1](https://github.com/meteroid-oss/meteroid-rust/compare/v0.30.0...v0.30.1) (2026-10-16)


### Bug Fixes

* **api:** update SDKs to meteroid 0.1.0 ([#14](https://github.com/meteroid-oss/meteroid-rust/issues/14)) ([ce050e1](https://github.com/meteroid-oss/meteroid-rust/commit/ce050e16f2b106b56a8ebefd3d59719cd5d36493))

## [0.30.0](https://github.com/meteroid-oss/meteroid-rust/compare/v0.29.0...v0.30.0) (2026-10-09)


### ⚠ BREAKING CHANGES

* **api:** update SDKs to meteroid 0.1.0 ([#12](https://github.com/meteroid-oss/meteroid-rust/issues/12))
* **api:** `POST /api/v1/events/ingest`: the `events/items/timestamp` request property `type/format` changed from `string` to `string, null/date-time`

### Features

* **api:** `POST /api/v1/events/ingest`: the `events/items/timestamp` request property `type/format` changed from `string` to `string, null/date-time` ([b5fb703](https://github.com/meteroid-oss/meteroid-rust/commit/b5fb7039dbec3285355854b7d5b8cb8e15eb520a))
* **api:** add `DELETE /api/v1/webhooks/endpoints/{endpoint_id}` ([b5fb703](https://github.com/meteroid-oss/meteroid-rust/commit/b5fb7039dbec3285355854b7d5b8cb8e15eb520a))
* **api:** update `GET /api/v1/plans` and 1029 more ([b5fb703](https://github.com/meteroid-oss/meteroid-rust/commit/b5fb7039dbec3285355854b7d5b8cb8e15eb520a))
* **api:** update SDKs to meteroid 0.1.0 ([#12](https://github.com/meteroid-oss/meteroid-rust/issues/12)) ([b5fb703](https://github.com/meteroid-oss/meteroid-rust/commit/b5fb7039dbec3285355854b7d5b8cb8e15eb520a))

## [0.29.0](https://github.com/meteroid-oss/meteroid-rust/compare/v0.28.0...v0.29.0) (2026-10-08)


### ⚠ BREAKING CHANGES

* **api:** update SDKs to meteroid 0.1.0 ([#10](https://github.com/meteroid-oss/meteroid-rust/issues/10))

### Features

* **api:** update SDKs to meteroid 0.1.0 ([#10](https://github.com/meteroid-oss/meteroid-rust/issues/10)) ([0aeaec1](https://github.com/meteroid-oss/meteroid-rust/commit/0aeaec1333c19f165bf843207a2cd1fd8b6b03ac))

## [0.28.0](https://github.com/meteroid-oss/meteroid-rust/compare/v0.27.1...v0.28.0) (2026-10-07)


### ⚠ BREAKING CHANGES

* **api:** update SDKs to meteroid 0.1.0 ([#8](https://github.com/meteroid-oss/meteroid-rust/issues/8))

### Features

* **api:** update SDKs to meteroid 0.1.0 ([#8](https://github.com/meteroid-oss/meteroid-rust/issues/8)) ([d0a7f3d](https://github.com/meteroid-oss/meteroid-rust/commit/d0a7f3db998e1ffe7be043a35242deec6d0de0f7))

## [0.27.1](https://github.com/meteroid-oss/meteroid-rust/compare/v0.27.0...v0.27.1) (2026-10-06)


### Features

* enable the webhook verifier by default ([#4](https://github.com/meteroid-oss/meteroid-rust/issues/4)) ([dc965ad](https://github.com/meteroid-oss/meteroid-rust/commit/dc965ad09ae870b6a1e3f15c7fbd4f17ad8b5969))

## [0.27.0](https://github.com/meteroid-oss/meteroid-rust/compare/v0.1.0...v0.27.0) (2026-10-06)


### Features

* **api:** update SDKs to meteroid 0.1.0 ([#1](https://github.com/meteroid-oss/meteroid-rust/issues/1)) ([8fbaa43](https://github.com/meteroid-oss/meteroid-rust/commit/8fbaa43eca7f8adf3055f041984ca8643ec6189b))


### Bug Fixes

* **api:** update SDKs to meteroid 0.1.0 ([#3](https://github.com/meteroid-oss/meteroid-rust/issues/3)) ([3dd18f6](https://github.com/meteroid-oss/meteroid-rust/commit/3dd18f61d9d7edb6dbc71324cd3d7d03e85b3f5b))
