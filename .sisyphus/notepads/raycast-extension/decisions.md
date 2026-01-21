# Architectural Decisions - Raycast Extension

## Task 2: API Client Design

### Decision 1: Single Class vs Multiple Modules
**Choice**: Single `ClerkAPI` class in one file

**Rationale**:
- All methods share same authentication and base URL
- Raycast extensions prefer simple, flat structure
- Easy to import: `import { ClerkAPI, getClerkClient } from "./api/clerk"`
- Total ~220 lines is manageable in single file

**Alternative Considered**: Separate files per resource (users.ts, orgs.ts)
- Rejected: Adds complexity for small API surface

### Decision 2: Generic Request Method
**Choice**: Private `request<T>()` method handles all HTTP logic

**Rationale**:
- DRY: Auth header, error handling, JSON parsing in one place
- Type-safe: Generic `<T>` ensures correct return types
- Matches Rust client pattern (all methods use same HTTP client)

**Implementation**:
```typescript
private async request<T>(endpoint: string, options?: RequestInit): Promise<T>
```

### Decision 3: Error Handling Strategy
**Choice**: Parse Clerk error format, throw Error with message

**Rationale**:
- Raycast shows error messages in toast notifications
- User-friendly messages more important than error codes
- Matches Rust client behavior (extract first error message)
- Fallback to HTTP status if JSON parsing fails

**Alternative Considered**: Custom error class with code property
- Rejected: Raycast doesn't expose error codes to user

### Decision 4: JWT Generation Implementation
**Choice**: Implement full two-step flow in `createUserJwt()`

**Rationale**:
- Clerk API requires active session to create JWT
- Hide complexity from command implementations
- Provide simple interface: `createUserJwt(userId, templateName)`
- Throw clear error if no active sessions

**Alternative Considered**: Expose `listSessions()` and `createSessionToken()` separately
- Rejected: Commands would need to implement same logic repeatedly

### Decision 5: Helper Functions for Display
**Choice**: Export standalone functions for name/email formatting

**Rationale**:
- Reusable across all command files
- Pure functions (no side effects)
- TypeScript-friendly (no prototype pollution)
- Matches functional programming style

**Functions**:
- `getUserDisplayName(user: User): string`
- `getUserPrimaryEmail(user: User): string | null`
- `getMemberDisplayName(member: Membership): string`

### Decision 6: Preferences Integration
**Choice**: Export `getClerkClient()` factory function

**Rationale**:
- Commands don't need to know about preferences
- Centralized API key validation
- Easy to mock for testing
- Clear error message if API key missing

**Usage Pattern**:
```typescript
const client = getClerkClient();
const users = await client.listUsers();
```

### Decision 7: TypeScript Interface Naming
**Choice**: Match Rust struct names exactly

**Rationale**:
- Consistency between Rust CLI and TypeScript extension
- Easy to reference Rust code when debugging
- Clear mapping: `User` (Rust) → `User` (TypeScript)

**Examples**:
- `User`, `Organization`, `Membership`
- `SignInToken`, `JwtTemplate`, `Session`
- `PublicUserData`, `EmailAddress`
