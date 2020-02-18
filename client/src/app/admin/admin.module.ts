import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { RoutingModule as AdminRouter } from './routing.module';
import { Page1Component as AdminPage1 } from './page1/page1.component';
import { Page2Component as AdminPage2 } from './page2/page2.component';
import { LayoutComponent } from './layout/layout.component';
import { NavbarComponent } from './partial/navbar/navbar.component';
import { ReactiveFormsModule, FormsModule } from '@angular/forms';
import { ProfileComponent } from './profile/profile.component';
import { ButtonSubmitComponent } from './partial/button-submit/button-submit.component';
import { ErrorDisplayComponent } from './partial/error-display/error-display.component';
import { UserComponent } from './user/user.component';
import { UserNewComponent } from './user/user-new/user-new.component';
import { UserDetailComponent } from './user/user-detail/user-detail.component';
import { FolderComponent } from './folder/folder.component';
import { FolderNewComponent } from './folder/folder-new/folder-new.component';
import { FolderDetailComponent } from './folder/folder-detail/folder-detail.component';
import { FolderEditComponent } from './folder/folder-edit/folder-edit.component';
import { DocumentNewComponent } from './document/document-new/document-new.component';
import { DocumentDetailComponent } from './document/document-detail/document-detail.component';

@NgModule({
  declarations: [
    AdminPage1,
    AdminPage2,
    LayoutComponent,
    NavbarComponent,
    ProfileComponent,
    ButtonSubmitComponent,
    ErrorDisplayComponent,
    UserComponent,
    UserNewComponent,
    UserDetailComponent,
    FolderComponent,
    FolderNewComponent,
    FolderDetailComponent,
    FolderEditComponent,
    DocumentNewComponent,
    DocumentDetailComponent,
  ],
  imports: [
    CommonModule,
    FormsModule,
    ReactiveFormsModule,
    AdminRouter,
  ]
})
export class AdminModule { }
