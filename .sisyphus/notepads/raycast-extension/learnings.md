# Learnings - Raycast Extension

## Task 2: Clerk API Client Implementation

### API Patterns Discovered

1. **Response Wrappers**:
   - Organizations endpoint returns: `{ data: Organization[] }`
   - Memberships endpoint returns: `{ data: Membership[] }`
   - Users endpoint returns: `User[]` directly (no wrapper)
   - Must handle both patterns in TypeScript client

2. **JWT Generation Flow**:
   - Requires two-step process:
     1. List user's active sessions: `GET /sessions?user_id={userId}&status=active`
     2. Create session token: `POST /sessions/{sessionId}/tokens/{templateName}`
   - Cannot directly create JWT without active session
   - Must handle "no active sessions" error case

3. **Error Handling Pattern**:
   - Clerk API returns: `{ errors: [{ message: string, code: string }] }`
   - Always use first error message for user-facing errors
   - Fallback to HTTP status text if JSON parsing fails

4. **TypeScript Conventions**:
   - Use `interface` for all data types (not `type`)
   - Use `class` for API client
   - Use `async/await` for all promises
   - Export helper function `getClerkClient()` for easy instantiation

5. **Raycast Preferences**:
   - Use `getPreferenceValues<Preferences>()` to read config
   - Preferences defined in package.json manifest
   - API key stored as `password` type (secure)
   - Frontend API optional for custom domains

### Code Organization

File structure:
```
src/api/clerk.ts
├── Preferences interface
├── Response type interfaces (User, Organization, etc.)
├── Error interfaces
├── Response wrapper interfaces
├── ClerkAPI class
│   ├── constructor(apiKey)
│   ├── private request<T>() - shared HTTP logic
│   ├── listOrganizations()
│   ├── listUsers()
│   ├── getOrganization()
│   ├── listOrgMembers()
│   ├── createSignInToken()
│   ├── listJwtTemplates()
│   └── createUserJwt() - complex two-step flow
├── getClerkClient() - helper factory
└── Display helpers (getUserDisplayName, etc.)
```

### TypeScript Best Practices Applied

1. **Strict typing**: No `any` types used
2. **Null safety**: All optional fields typed as `string | null`
3. **Generic request method**: Reusable `request<T>()` for all API calls
4. **Error propagation**: Throw typed errors, let caller handle
5. **Helper functions**: Extracted display logic for reuse

### Dependencies Used

- `@raycast/api`: For `getPreferenceValues()`
- Built-in `fetch()`: No external HTTP library needed
- TypeScript 5.2.2: Modern async/await support

### Verification Steps

1. ✅ TypeScript compilation: `npm run build` succeeds
2. ✅ LSP diagnostics: No errors or warnings
3. ✅ All 7 required methods implemented
4. ✅ All response types properly defined
5. ✅ Error handling matches Rust client behavior
