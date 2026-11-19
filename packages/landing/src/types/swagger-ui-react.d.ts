declare module 'swagger-ui-react' {
  import type { ComponentType } from 'react';

  const SwaggerUI: ComponentType<Record<string, unknown>>;
  export default SwaggerUI;
}

declare module 'swagger-ui-react/swagger-ui.css';
