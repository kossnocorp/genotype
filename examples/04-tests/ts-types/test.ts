import assert from "node:assert/strict";

import type {
    Base as AliasBase,
    Envelope as AliasEnvelope,
    Payload as AliasPayload,
} from "genotype-test-ts-types-alias";
import type {
    Base as InterfaceBase,
    Envelope as InterfaceEnvelope,
    Payload as InterfacePayload,
} from "genotype-test-ts-types-interface";

const interfaceBase: InterfaceBase = {
  id: "evt_123",
};

const interfacePayload: InterfacePayload = {
  ...interfaceBase,
  kind: "payload",
  title: "Build completed",
  enabled: true,
};

const aliasBase: AliasBase = {
  id: "evt_456",
};

const aliasPayload: AliasPayload = {
  ...aliasBase,
  kind: "payload",
  title: "Release created",
  enabled: false,
};

const interfaceEnvelope: InterfaceEnvelope = interfacePayload;
const aliasEnvelope: AliasEnvelope = aliasPayload;

assert.equal(interfaceEnvelope.kind, "payload");
assert.equal(aliasEnvelope.kind, "payload");

