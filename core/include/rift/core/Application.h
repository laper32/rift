#ifndef RIFT_CORE_APPLICATION_H
#define RIFT_CORE_APPLICATION_H

#pragma once

enum EApplicationSearchPath
{
	Installation,
	UserProfile,
	Project
};

const char* GetApplicationSearchPath(EApplicationSearchPath search_path);


#endif // !RIFT_CORE_APPLICATION_H
