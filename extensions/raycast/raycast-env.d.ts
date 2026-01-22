/// <reference types="@raycast/api">

/* 🚧 🚧 🚧
 * This file is auto-generated from the extension's manifest.
 * Do not modify manually. Instead, update the `package.json` file.
 * 🚧 🚧 🚧 */

/* eslint-disable @typescript-eslint/ban-types */

type ExtensionPreferences = {
  /** Clerk API Key - Your Clerk secret key (sk_test_... or sk_live_...) */
  "clerkApiKey": string,
  /** Clerk Frontend API - Frontend API URL (required for org-scoped JWTs and impersonation) */
  "clerkFrontendApi": string
}

/** Preferences accessible in all the extension's commands */
declare type Preferences = ExtensionPreferences

declare namespace Preferences {
  /** Preferences accessible in the `search-organizations` command */
  export type SearchOrganizations = ExtensionPreferences & {}
  /** Preferences accessible in the `search-users` command */
  export type SearchUsers = ExtensionPreferences & {}
  /** Preferences accessible in the `impersonate-user` command */
  export type ImpersonateUser = ExtensionPreferences & {}
  /** Preferences accessible in the `generate-jwt` command */
  export type GenerateJwt = ExtensionPreferences & {}
  /** Preferences accessible in the `org-members` command */
  export type OrgMembers = ExtensionPreferences & {}
  /** Preferences accessible in the `switch-instance` command */
  export type SwitchInstance = ExtensionPreferences & {}
}

declare namespace Arguments {
  /** Arguments passed to the `search-organizations` command */
  export type SearchOrganizations = {}
  /** Arguments passed to the `search-users` command */
  export type SearchUsers = {}
  /** Arguments passed to the `impersonate-user` command */
  export type ImpersonateUser = {}
  /** Arguments passed to the `generate-jwt` command */
  export type GenerateJwt = {}
  /** Arguments passed to the `org-members` command */
  export type OrgMembers = {}
  /** Arguments passed to the `switch-instance` command */
  export type SwitchInstance = {}
}

